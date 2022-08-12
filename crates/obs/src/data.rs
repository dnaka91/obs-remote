use std::{marker::PhantomData, os::raw::c_char, ptr::NonNull};

use anyhow::{ensure, Result};

use crate::{cstr, cstr_ptr, util::StringConversion};

pub struct Data<'a> {
    raw: NonNull<libobs_sys::obs_data_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for Data<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_data_release(self.raw.as_ptr()) };
    }
}

impl<'a> Default for Data<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Data<'a> {
    pub fn new() -> Self {
        Self::from_raw(unsafe { libobs_sys::obs_data_create() })
    }

    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_data_t) -> Self {
        Self {
            raw: { unsafe { NonNull::new_unchecked(raw) } },
            life: PhantomData::default(),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_data_t {
        self.raw.as_ptr()
    }

    pub fn clear(&mut self) {
        unsafe { libobs_sys::obs_data_clear(self.raw.as_ptr()) };
    }

    pub fn apply(&mut self, apply_data: &Self) {
        unsafe { libobs_sys::obs_data_apply(self.raw.as_ptr(), apply_data.raw.as_ptr()) };
    }

    pub fn to_json(&self) -> String {
        unsafe { libobs_sys::obs_data_get_json(self.raw.as_ptr()) }.into_string()
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let raw = unsafe { libobs_sys::obs_data_create_from_json(cstr_ptr!(json)) };

        ensure!(!raw.is_null(), "invalid JSON");

        Ok(Self::from_raw(raw))
    }

    pub fn item_by_name(&self, name: &str) -> Option<DataItem> {
        let raw = unsafe { libobs_sys::obs_data_item_byname(self.raw.as_ptr(), cstr_ptr!(name)) };

        (!raw.is_null()).then(|| DataItem::from_raw(raw))
    }

    pub fn bool(&self, name: &str) -> Option<bool> {
        self.get(name, libobs_sys::obs_data_get_bool)
    }

    pub fn string(&self, name: &str) -> Option<String> {
        self.get(name, libobs_sys::obs_data_get_string)
            .map(|value| value.into_string())
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        self.get(name, libobs_sys::obs_data_get_int)
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.set(name, value, libobs_sys::obs_data_set_bool)
    }

    pub fn set_string(&mut self, name: &str, value: &str) {
        self.set(name, cstr_ptr!(value), libobs_sys::obs_data_set_string)
    }

    pub fn set_int(&mut self, name: &str, value: i64) {
        self.set(name, value, libobs_sys::obs_data_set_int)
    }

    pub fn erase(&mut self, name: &str) {
        unsafe { libobs_sys::obs_data_erase(self.raw.as_ptr(), cstr_ptr!(name)) };
    }

    fn get<T>(
        &self,
        name: &str,
        f: unsafe extern "C" fn(*mut libobs_sys::obs_data_t, *const c_char) -> T,
    ) -> Option<T> {
        let name = cstr!(name);

        unsafe { libobs_sys::obs_data_has_user_value(self.raw.as_ptr(), name.as_ptr()) }
            .then(|| unsafe { f(self.raw.as_ptr(), name.as_ptr()) })
    }

    fn set<T>(
        &mut self,
        name: &str,
        value: T,
        f: unsafe extern "C" fn(*mut libobs_sys::obs_data_t, *const c_char, T),
    ) {
        unsafe { f(self.raw.as_ptr(), cstr_ptr!(name), value) };
    }
}

pub struct DataArray {
    raw: NonNull<libobs_sys::obs_data_array_t>,
}

impl Drop for DataArray {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_data_array_release(self.raw.as_ptr()) }
    }
}

impl Default for DataArray {
    fn default() -> Self {
        Self::new()
    }
}

impl DataArray {
    fn from_raw(raw: *mut libobs_sys::obs_data_array_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn new() -> Self {
        Self::from_raw(unsafe { libobs_sys::obs_data_array_create() })
    }

    pub fn len(&self) -> u64 {
        unsafe { libobs_sys::obs_data_array_count(self.raw.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn erase(&mut self, idx: u64) {
        unsafe { libobs_sys::obs_data_array_erase(self.raw.as_ptr(), idx) };
    }

    pub fn insert(&mut self, idx: u64, obj: Data<'_>) {
        unsafe { libobs_sys::obs_data_array_insert(self.raw.as_ptr(), idx, obj.raw.as_ptr()) };
    }

    pub fn get(&self, idx: u64) -> Data<'_> {
        Data::from_raw(unsafe { libobs_sys::obs_data_array_item(self.raw.as_ptr(), idx) })
    }

    pub fn push_back(&mut self, obj: Data<'_>) {
        unsafe { libobs_sys::obs_data_array_push_back(self.raw.as_ptr(), obj.raw.as_ptr()) };
    }

    pub fn push_back_array(&mut self, array: Self) {
        unsafe {
            libobs_sys::obs_data_array_push_back_array(self.raw.as_ptr(), array.raw.as_ptr())
        };
    }
}

pub struct DataItem {
    raw: NonNull<libobs_sys::obs_data_item_t>,
}

impl DataItem {
    fn from_raw(raw: *mut libobs_sys::obs_data_item_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn ty(&self) -> DataType {
        DataType::from_native(unsafe { libobs_sys::obs_data_item_gettype(self.raw.as_ptr()) })
    }

    pub fn numtype(&self) -> DataNumberType {
        DataNumberType::from_native(unsafe { libobs_sys::obs_data_item_numtype(self.raw.as_ptr()) })
    }

    pub fn string(&self) -> Option<String> {
        unsafe { libobs_sys::obs_data_item_get_string(self.raw.as_ptr()) }.into_opt_string()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Null,
    String,
    Number,
    Boolean,
    Object,
    Array,
    Unknown(u32),
}

impl DataType {
    fn from_native(ty: libobs_sys::obs_data_type::Type) -> Self {
        use libobs_sys::obs_data_type::*;

        match ty {
            OBS_DATA_NULL => Self::Null,
            OBS_DATA_STRING => Self::String,
            OBS_DATA_NUMBER => Self::Number,
            OBS_DATA_BOOLEAN => Self::Boolean,
            OBS_DATA_OBJECT => Self::Object,
            OBS_DATA_ARRAY => Self::Array,
            _ => Self::Unknown(ty as _),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum DataNumberType {
    Invalid,
    Int,
    Double,
    Unknown(u32),
}

impl DataNumberType {
    fn from_native(ty: libobs_sys::obs_data_number_type::Type) -> Self {
        use libobs_sys::obs_data_number_type::*;

        match ty {
            OBS_DATA_NUM_INVALID => Self::Invalid,
            OBS_DATA_NUM_INT => Self::Int,
            OBS_DATA_NUM_DOUBLE => Self::Double,
            _ => Self::Unknown(ty as _),
        }
    }
}
