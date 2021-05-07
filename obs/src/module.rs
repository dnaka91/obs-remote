use std::{ffi::c_void, path::PathBuf, ptr::NonNull};

use crate::{cstr_ptr, util::StringConversion};

pub struct Module {
    raw: NonNull<libobs_sys::obs_module_t>,
}

impl Module {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_module_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    /// Return a module based upon its name.
    pub fn by_name(name: &str) -> Option<Self> {
        let raw = unsafe { libobs_sys::obs_get_module(cstr_ptr!(name)) };
        NonNull::new(raw).map(|raw| Module { raw })
    }

    /// Returns the module author(s).
    pub fn author(&self) -> Option<String> {
        unsafe { libobs_sys::obs_get_module_author(self.raw.as_ptr()) }.into_opt_string()
    }

    /// Returns the module binary path.
    pub fn binary_path(&self) -> PathBuf {
        unsafe { libobs_sys::obs_get_module_binary_path(self.raw.as_ptr()) }
            .into_string()
            .into()
    }

    /// Returns the module data path.
    pub fn data_path(&self) -> PathBuf {
        unsafe { libobs_sys::obs_get_module_data_path(self.raw.as_ptr()) }
            .into_string()
            .into()
    }

    /// Returns the module description.
    pub fn description(&self) -> Option<String> {
        unsafe { libobs_sys::obs_get_module_description(self.raw.as_ptr()) }.into_opt_string()
    }

    /// Returns the module file name.
    pub fn file_name(&self) -> String {
        unsafe { libobs_sys::obs_get_module_file_name(self.raw.as_ptr()) }.into_string()
    }

    /// Returns the module full name.
    pub fn name(&self) -> Option<String> {
        unsafe { libobs_sys::obs_get_module_name(self.raw.as_ptr()) }.into_opt_string()
    }
}

pub fn list_modules() -> Vec<Module> {
    unsafe extern "C" fn callback(param: *mut c_void, module: *mut libobs_sys::obs_module_t) {
        if !module.is_null() {
            let param = &mut *param.cast::<Vec<Module>>();
            param.push(Module::from_raw(module));
        }
    }

    let mut modules = Vec::<Module>::new();
    unsafe { libobs_sys::obs_enum_modules(Some(callback), (&mut modules as *mut Vec<_>).cast()) };

    modules
}
