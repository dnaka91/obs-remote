use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    path::PathBuf,
    ptr,
};

pub trait FfiToString {
    fn into_string(self) -> String;
    fn into_opt_string(self) -> Option<String>;
    fn into_path_buf(self) -> PathBuf;
}

impl FfiToString for *const c_char {
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
}

pub trait StringToFfi {
    fn cstr(self) -> CString;
}

impl<T> StringToFfi for T
where
    T: Into<Vec<u8>>,
{
    #[inline]
    fn cstr(self) -> CString {
        CString::new(self.into()).expect("invalid string containing '0' bytes")
    }
}

pub fn list_types(f: unsafe extern "C" fn(usize, *mut *const c_char) -> bool) -> Vec<String> {
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
            let param = unsafe { &mut *param.cast::<Param<R, T>>() };
            let r = param.get_ref;
            let f = param.converter;
            param.instances.push(f(unsafe { r(raw) }));
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
            let param = unsafe { &mut *param.cast::<Param<C, T>>() };
            let r = param.get_ref;
            let f = param.converter;
            param.instances.push(f(unsafe { r(child) }));
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

pub fn convert_string_list(raw: *mut *const c_char) -> Vec<String> {
    if raw.is_null() {
        return Vec::new();
    }

    let mut index = 0;
    let mut values = Vec::new();

    loop {
        let value = unsafe { *raw.add(index) };
        if value.is_null() {
            break values;
        }

        values.push(value.into_string());
        index += 1;
    }
}

pub fn convert_string_list_mut(raw: *mut *mut c_char) -> Vec<String> {
    let list = convert_string_list(raw as *mut *const _);
    unsafe { libobs_sys::bfree(raw as *mut _) };

    list
}
