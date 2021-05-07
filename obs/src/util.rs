use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
    ptr,
};

pub trait StringConversion {
    fn into_string(self) -> String;
    fn into_opt_string(self) -> Option<String>;
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
