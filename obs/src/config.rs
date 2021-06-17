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

    /// Gets a string value.
    pub fn string(&self, section: &str, name: &str) -> Option<String> {
        self.get(section, name, libobs_sys::config_get_string)
            .map(|value| value.into_string())
    }

    /// Gets an integer value.
    pub fn int(&self, section: &str, name: &str) -> Option<i64> {
        self.get(section, name, libobs_sys::config_get_int)
    }

    /// Gets an unsigned integer value.
    pub fn uint(&self, section: &str, name: &str) -> Option<u64> {
        self.get(section, name, libobs_sys::config_get_uint)
    }

    /// Gets a boolean value.
    pub fn bool(&self, section: &str, name: &str) -> Option<bool> {
        self.get(section, name, libobs_sys::config_get_bool)
    }

    /// Gets a floating point value.
    pub fn double(&self, section: &str, name: &str) -> Option<f64> {
        self.get(section, name, libobs_sys::config_get_double)
    }

    /// Gets a default string value.
    pub fn default_string(&self, section: &str, name: &str) -> Option<String> {
        self.get_default(section, name, libobs_sys::config_get_default_string)
            .map(|value| value.into_string())
    }

    /// Gets a default integer value.
    pub fn default_int(&self, section: &str, name: &str) -> Option<i64> {
        self.get_default(section, name, libobs_sys::config_get_default_int)
    }

    /// Gets a default unsigned integer value.
    pub fn default_uint(&self, section: &str, name: &str) -> Option<u64> {
        self.get_default(section, name, libobs_sys::config_get_default_uint)
    }

    /// Gets a default boolean value.
    pub fn default_bool(&self, section: &str, name: &str) -> Option<bool> {
        self.get_default(section, name, libobs_sys::config_get_default_bool)
    }

    /// Gets a default floating point value.
    pub fn default_double(&self, section: &str, name: &str) -> Option<f64> {
        self.get_default(section, name, libobs_sys::config_get_default_double)
    }

    /// Sets a string value.
    pub fn set_string(&self, section: &str, name: &str, value: &str) {
        self.set(
            section,
            name,
            cstr_ptr!(value),
            libobs_sys::config_set_string,
        )
    }

    /// Sets an integer value.
    pub fn set_int(&self, section: &str, name: &str, value: i64) {
        self.set(section, name, value, libobs_sys::config_set_int)
    }

    /// Sets an unsigned integer value.
    pub fn set_uint(&self, section: &str, name: &str, value: u64) {
        self.set(section, name, value, libobs_sys::config_set_uint)
    }

    /// Sets a boolean value.
    pub fn set_bool(&self, section: &str, name: &str, value: bool) {
        self.set(section, name, value, libobs_sys::config_set_bool)
    }

    /// Sets a floating point value.
    pub fn set_double(&self, section: &str, name: &str, value: f64) {
        self.set(section, name, value, libobs_sys::config_set_double)
    }

    /// Sets a default string value.
    pub fn set_default_string(&self, section: &str, name: &str, value: &str) {
        self.set(
            section,
            name,
            cstr_ptr!(value),
            libobs_sys::config_set_default_string,
        )
    }

    /// Sets a default integer value.
    pub fn set_default_int(&self, section: &str, name: &str, value: i64) {
        self.set(section, name, value, libobs_sys::config_set_default_int)
    }

    /// Sets a default unsigned integer value.
    pub fn set_default_uint(&self, section: &str, name: &str, value: u64) {
        self.set(section, name, value, libobs_sys::config_set_default_uint)
    }

    /// Sets a default boolean value.
    pub fn set_default_bool(&self, section: &str, name: &str, value: bool) {
        self.set(section, name, value, libobs_sys::config_set_default_bool)
    }

    /// Sets a default floating point value.
    pub fn set_default_double(&self, section: &str, name: &str, value: f64) {
        self.set(section, name, value, libobs_sys::config_set_default_double)
    }

    pub fn remove_value(&self, section: &str, name: &str) -> bool {
        unsafe {
            libobs_sys::config_remove_value(self.raw.as_ptr(), cstr_ptr!(section), cstr_ptr!(name))
        }
    }

    /// Creates a new configuration object and associates it with the specified file name.
    pub fn create(file: &str) -> Self {
        Self::from_raw(unsafe { libobs_sys::config_create(cstr_ptr!(file)) })
    }

    /// Opens a configuration file.
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

    /// Saves configuration data to a file (if associated with a file).
    pub fn save(&self) -> Result<()> {
        let res = unsafe { libobs_sys::config_save(self.raw.as_ptr()) };

        ensure!(
            res == libobs_sys::CONFIG_SUCCESS as i32,
            "failed saving config"
        );
        Ok(())
    }

    /// Closes the configuration object.
    pub fn close(self) {
        unsafe { libobs_sys::config_close(self.raw.as_ptr()) };
    }

    /// Returns the number of sections.
    pub fn sections(&self) -> u64 {
        unsafe { libobs_sys::config_num_sections(self.raw.as_ptr()) }
    }

    /// Returns a section name based upon its index.
    pub fn section(&self, idx: u64) -> Option<String> {
        unsafe { libobs_sys::config_get_section(self.raw.as_ptr(), idx) }.into_opt_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OpenType {
    /// Try to open the file. If it doesn't exist, create it.
    Always,
    /// Fail if the file doesn't exist.
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
