use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
    path::PathBuf,
    ptr,
};

pub trait StringConversion {
    fn into_string(self) -> String;
    fn into_opt_string(self) -> Option<String>;
    fn into_path_buf(self) -> PathBuf;
    fn into_opt_path_buf(self) -> Option<PathBuf>;
}

impl StringConversion for *const c_char {
    fn into_string(self) -> String {
        unsafe { CStr::from_ptr(self) }
            .to_string_lossy()
            .into_owned()
    }

    fn into_opt_string(self) -> Option<String> {
        if self.is_null() {
            None
        } else {
            Some(self.into_string())
        }
    }

    fn into_path_buf(self) -> PathBuf {
        // TODO: Probably there is a better way of transforming as `CStr -> OsStr -> Path`,
        // allowing to have non-UTF8 content.
        self.into_string().into()
    }

    fn into_opt_path_buf(self) -> Option<PathBuf> {
        self.into_opt_string().map(Into::into)
    }
}

pub fn list_types(
    f: unsafe extern "C" fn(libobs_sys::size_t, *mut *const c_char) -> bool,
) -> Vec<String> {
    let mut id = ptr::null::<c_char>();
    let raw = (&mut id) as *mut _;
    let mut idx = 0;
    let mut values = Vec::new();

    while unsafe { f(idx, raw) } {
        values.push(id.into_string());
        idx += 1;
    }

    values
}

pub fn list_instances<R, T>(
    f: unsafe extern "C" fn(Option<unsafe extern "C" fn(*mut c_void, *mut R) -> bool>, *mut c_void),
    get_ref: unsafe extern "C" fn(*mut R) -> *mut R,
    converter: fn(*mut R) -> T,
) -> Vec<T> {
    struct Param<R, T> {
        instances: Vec<T>,
        get_ref: unsafe extern "C" fn(*mut R) -> *mut R,
        converter: fn(*mut R) -> T,
    }

    unsafe extern "C" fn callback<R, T>(param: *mut c_void, raw: *mut R) -> bool {
        if !raw.is_null() {
            let param = &mut *param.cast::<Param<R, T>>();
            let r = param.get_ref;
            let f = param.converter;
            param.instances.push(f(r(raw)));
        }

        true
    }

    let mut param = Param {
        instances: Vec::new(),
        get_ref,
        converter,
    };

    unsafe {
        f(
            Some(callback::<R, T>),
            (&mut param as *mut Param<_, _>).cast(),
        )
    };

    param.instances
}

pub fn list_instances_of<P, C, T>(
    parent: *mut P,
    f: unsafe extern "C" fn(
        *mut P,
        Option<unsafe extern "C" fn(*mut P, *mut C, *mut c_void)>,
        *mut c_void,
    ),
    get_ref: unsafe extern "C" fn(*mut C) -> *mut C,
    converter: fn(*mut C) -> T,
) -> Vec<T> {
    struct Param<C, T> {
        instances: Vec<T>,
        get_ref: unsafe extern "C" fn(*mut C) -> *mut C,
        converter: fn(*mut C) -> T,
    }

    unsafe extern "C" fn callback<P, C, T>(_parent: *mut P, child: *mut C, param: *mut c_void) {
        if !child.is_null() {
            let param = &mut *param.cast::<Param<C, T>>();
            let r = param.get_ref;
            let f = param.converter;
            param.instances.push(f(r(child)));
        }
    }

    let mut param = Param {
        instances: Vec::new(),
        get_ref,
        converter,
    };

    unsafe {
        f(
            parent,
            Some(callback::<P, C, T>),
            (&mut param as *mut Param<_, _>).cast(),
        )
    };

    param.instances
}

pub fn find_instance_of<P, C, T>(
    parent: *mut P,
    search: *mut C,
    f: unsafe extern "C" fn(
        *mut P,
        Option<unsafe extern "C" fn(*mut P, *mut C, *mut c_void)>,
        *mut c_void,
    ),
    get_ref: unsafe extern "C" fn(*mut C) -> *mut C,
    converter: fn(*mut C) -> T,
) -> Option<(usize, T)> {
    struct Param<C, T> {
        search: *mut C,
        found: Option<T>,
        index: usize,
        get_ref: unsafe extern "C" fn(*mut C) -> *mut C,
        converter: fn(*mut C) -> T,
    }

    unsafe extern "C" fn callback<P, C, T>(_parent: *mut P, child: *mut C, param: *mut c_void) {
        if !child.is_null() {
            let param = &mut *param.cast::<Param<C, T>>();
            let r = param.get_ref;
            let f = param.converter;

            if param.found.is_none() && child == param.search {
                param.found = Some(f(r(child)));
            }

            if param.found.is_none() {
                param.index += 1;
            }
        }
    }

    let mut param = Param {
        search,
        found: None,
        index: 0,
        get_ref,
        converter,
    };

    unsafe {
        f(
            parent,
            Some(callback::<P, C, T>),
            (&mut param as *mut Param<_, _>).cast(),
        )
    };

    param.found.map(|found| (param.index, found))
}
