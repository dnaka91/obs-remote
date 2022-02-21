use std::fmt::{self, Debug};

use super::quat::Quat;

#[derive(Clone, Copy, Default)]
pub struct Axisang(libobs_sys::axisang);

impl Axisang {
    pub(crate) fn as_ptr(&self) -> *const libobs_sys::axisang {
        &self.0 as _
    }

    fn as_ptr_mut(&mut self) -> *mut libobs_sys::axisang {
        &mut self.0 as _
    }

    #[inline]
    pub fn x(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1.x }
    }

    #[inline]
    pub fn y(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1.y }
    }

    #[inline]
    pub fn z(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1.z }
    }

    #[inline]
    pub fn w(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.__bindgen_anon_1.w }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(libobs_sys::axisang {
            __bindgen_anon_1: libobs_sys::axisang__bindgen_ty_1 {
                __bindgen_anon_1: libobs_sys::axisang__bindgen_ty_1__bindgen_ty_1 { x, y, z, w },
            },
        })
    }
}

impl Debug for Axisang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Axisang")
            .field(&self.x())
            .field(&self.y())
            .field(&self.z())
            .field(&self.w())
            .finish()
    }
}

impl From<Quat> for Axisang {
    fn from(value: Quat) -> Self {
        let mut dst = Axisang::default();
        unsafe { libobs_sys::axisang_from_quat(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}
