use std::{os::raw::c_char, ptr::NonNull};

use anyhow::{bail, ensure, Result};

use crate::{cstr, cstr_ptr, util::StringConversion};

pub struct Config {
    raw: NonNull<libobs_sys::config_t>,
}

impl Config {
    pub(crate) fn from_raw(raw: *mut libobs_sys::config_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    fn get<T>(
        &self,
        section: &str,
        name: &str,
        f: unsafe extern "C" fn(*mut libobs_sys::config_t, *const c_char, *const c_char) -> T,
    ) -> Option<T> {
        let section = cstr!(section);
        let name = cstr!(name);

        unsafe {
            libobs_sys::config_has_user_value(self.raw.as_ptr(), section.as_ptr(), name.as_ptr())
        }
        .then(|| unsafe { f(self.raw.as_ptr(), section.as_ptr(), name.as_ptr()) })
    }

    fn get_default<T>(
        &self,
        section: &str,
        name: &str,
        f: unsafe extern "C" fn(*mut libobs_sys::config_t, *const c_char, *const c_char) -> T,
    ) -> Option<T> {
        let section = cstr!(section);
        let name = cstr!(name);

        unsafe {
            libobs_sys::config_has_default_value(self.raw.as_ptr(), section.as_ptr(), name.as_ptr())
        }
        .then(|| unsafe { f(self.raw.as_ptr(), section.as_ptr(), name.as_ptr()) })
    }

    fn set<T>(
        &self,
        section: &str,
        name: &str,
        value: T,
        f: unsafe extern "C" fn(*mut libobs_sys::config_t, *const c_char, *const c_char, T),
    ) {
        unsafe {
            f(
                self.raw.as_ptr(),
                cstr_ptr!(section),
                cstr_ptr!(name),
                value,
            )
        };
    }

    pub fn bool(&self, section: &str, name: &str) -> Option<bool> {
        self.get(section, name, libobs_sys::config_get_bool)
    }

    pub fn double(&self, section: &str, name: &str) -> Option<f64> {
        self.get(section, name, libobs_sys::config_get_double)
    }

    pub fn int(&self, section: &str, name: &str) -> Option<i64> {
        self.get(section, name, libobs_sys::config_get_int)
    }

    pub fn string(&self, section: &str, name: &str) -> Option<String> {
        self.get(section, name, libobs_sys::config_get_string)
            .map(|value| value.into_string())
    }

    pub fn uint(&self, section: &str, name: &str) -> Option<u64> {
        self.get(section, name, libobs_sys::config_get_uint)
    }

    pub fn default_bool(&self, section: &str, name: &str) -> Option<bool> {
        self.get_default(section, name, libobs_sys::config_get_default_bool)
    }

    pub fn default_double(&self, section: &str, name: &str) -> Option<f64> {
        self.get_default(section, name, libobs_sys::config_get_default_double)
    }

    pub fn default_int(&self, section: &str, name: &str) -> Option<i64> {
        self.get_default(section, name, libobs_sys::config_get_default_int)
    }

    pub fn default_string(&self, section: &str, name: &str) -> Option<String> {
        self.get_default(section, name, libobs_sys::config_get_default_string)
            .map(|value| value.into_string())
    }

    pub fn default_uint(&self, section: &str, name: &str) -> Option<u64> {
        self.get_default(section, name, libobs_sys::config_get_default_uint)
    }

    pub fn set_bool(&self, section: &str, name: &str, value: bool) {
        self.set(section, name, value, libobs_sys::config_set_bool)
    }

    pub fn set_double(&self, section: &str, name: &str, value: f64) {
        self.set(section, name, value, libobs_sys::config_set_double)
    }

    pub fn set_int(&self, section: &str, name: &str, value: i64) {
        self.set(section, name, value, libobs_sys::config_set_int)
    }

    pub fn set_string(&self, section: &str, name: &str, value: &str) {
        self.set(
            section,
            name,
            cstr_ptr!(value),
            libobs_sys::config_set_string,
        )
    }

    pub fn set_uint(&self, section: &str, name: &str, value: u64) {
        self.set(section, name, value, libobs_sys::config_set_uint)
    }

    pub fn set_default_bool(&self, section: &str, name: &str, value: bool) {
        self.set(section, name, value, libobs_sys::config_set_default_bool)
    }

    pub fn set_default_double(&self, section: &str, name: &str, value: f64) {
        self.set(section, name, value, libobs_sys::config_set_default_double)
    }

    pub fn set_default_int(&self, section: &str, name: &str, value: i64) {
        self.set(section, name, value, libobs_sys::config_set_default_int)
    }

    pub fn set_default_string(&self, section: &str, name: &str, value: &str) {
        self.set(
            section,
            name,
            cstr_ptr!(value),
            libobs_sys::config_set_default_string,
        )
    }

    pub fn set_default_uint(&self, section: &str, name: &str, value: u64) {
        self.set(section, name, value, libobs_sys::config_set_default_uint)
    }

    pub fn remove_value(&self, section: &str, name: &str) -> bool {
        unsafe {
            libobs_sys::config_remove_value(self.raw.as_ptr(), cstr_ptr!(section), cstr_ptr!(name))
        }
    }

    pub fn create(file: &str) -> Self {
        Self::from_raw(unsafe { libobs_sys::config_create(cstr_ptr!(file)) })
    }

    pub fn open(file: &str, open_type: OpenType) -> Result<Self> {
        const CONFIG_SUCCESS: i32 = libobs_sys::CONFIG_SUCCESS as i32;

        let mut config = std::ptr::null_mut::<libobs_sys::config_t>();

        let res = unsafe {
            libobs_sys::config_open(
                &mut config as *mut _,
                cstr_ptr!(file),
                open_type.to_native(),
            )
        };

        match res {
            CONFIG_SUCCESS if !config.is_null() => Ok(Self::from_raw(config)),
            libobs_sys::CONFIG_FILENOTFOUND => bail!("file at `{}` not found", file),
            _ => bail!("failed opening config at `{}`", file),
        }
    }

    pub fn save(&self) -> Result<()> {
        let res = unsafe { libobs_sys::config_save(self.raw.as_ptr()) };

        ensure!(
            res == libobs_sys::CONFIG_SUCCESS as i32,
            "failed saving config"
        );
        Ok(())
    }

    pub fn close(self) {
        unsafe { libobs_sys::config_close(self.raw.as_ptr()) };
    }

    pub fn sections(&self) -> u64 {
        unsafe { libobs_sys::config_num_sections(self.raw.as_ptr()) }
    }

    pub fn section(&self, idx: u64) -> Option<String> {
        unsafe { libobs_sys::config_get_section(self.raw.as_ptr(), idx) }.into_opt_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OpenType {
    Always,
    Existing,
}

impl OpenType {
    fn to_native(self) -> libobs_sys::config_open_type::Type {
        use libobs_sys::config_open_type::*;

        match self {
            Self::Always => CONFIG_OPEN_ALWAYS,
            Self::Existing => CONFIG_OPEN_EXISTING,
        }
    }
}
