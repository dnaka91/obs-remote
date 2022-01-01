use std::{
    fmt::{self, Debug},
    ops::Mul,
};

use super::Matrix3;
use crate::graphics::{Axisang, Quat, Vec3, Vec4};

#[derive(Clone, Copy, Default)]
pub struct Matrix4(libobs_sys::matrix4);

impl Matrix4 {
    pub(crate) fn as_ptr(&self) -> *const libobs_sys::matrix4 {
        &self.0 as _
    }

    fn as_ptr_mut(&mut self) -> *mut libobs_sys::matrix4 {
        &mut self.0 as _
    }

    #[inline]
    pub fn identity() -> Self {
        let mut new = Self::default();
        new.0.x.__bindgen_anon_1.__bindgen_anon_1.x = 1.0;
        new.0.y.__bindgen_anon_1.__bindgen_anon_1.y = 1.0;
        new.0.z.__bindgen_anon_1.__bindgen_anon_1.z = 1.0;
        new.0.t.__bindgen_anon_1.__bindgen_anon_1.w = 1.0;
        new
    }

    pub fn determinant(self) -> f32 {
        unsafe { libobs_sys::matrix4_determinant(self.as_ptr()) }
    }

    #[must_use]
    pub fn translate3v(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_translate3v(dst.as_ptr_mut(), self.as_ptr(), v.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn translate4v(self, v: Vec4) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_translate4v(dst.as_ptr_mut(), self.as_ptr(), v.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn rotate(self, q: Quat) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_rotate(dst.as_ptr_mut(), self.as_ptr(), q.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn rotate_aa(self, aa: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_rotate_aa(dst.as_ptr_mut(), self.as_ptr(), aa.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn scale(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_scale(dst.as_ptr_mut(), self.as_ptr(), v.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn inv(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_inv(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn transpose(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_transpose(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn translate3v_i(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_translate3v_i(dst.as_ptr_mut(), v.as_ptr(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn translate4v_i(self, v: Vec4) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_translate4v_i(dst.as_ptr_mut(), v.as_ptr(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn rotate_i(self, q: Quat) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_rotate_i(dst.as_ptr_mut(), q.as_ptr(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn rotate_aa_i(self, aa: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_rotate_aa_i(dst.as_ptr_mut(), aa.as_ptr(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn scale_i(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_scale_i(dst.as_ptr_mut(), v.as_ptr(), self.as_ptr()) };
        dst
    }

    #[inline]
    #[must_use]
    pub fn translate3f(self, x: f32, y: f32, z: f32) -> Self {
        self.translate3v(Vec3::new(x, y, z))
    }

    #[inline]
    #[must_use]
    pub fn rotate_aa4f(self, x: f32, y: f32, z: f32, rot: f32) -> Self {
        self.rotate_aa(Axisang::new(x, y, z, rot))
    }

    #[inline]
    #[must_use]
    pub fn scale3f(self, x: f32, y: f32, z: f32) -> Self {
        self.scale(Vec3::new(x, y, z))
    }
}

impl Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matrix4")
            .field(&Vec4::from_raw(self.0.x))
            .field(&Vec4::from_raw(self.0.y))
            .field(&Vec4::from_raw(self.0.z))
            .field(&Vec4::from_raw(self.0.t))
            .finish()
    }
}

impl From<Matrix3> for Matrix4 {
    fn from(value: Matrix3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_from_matrix3(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Quat> for Matrix4 {
    fn from(value: Quat) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_from_quat(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Axisang> for Matrix4 {
    fn from(value: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_from_axisang(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix4_mul(dst.as_ptr_mut(), self.as_ptr(), rhs.as_ptr()) };
        dst
    }
}

impl From<(Vec4, Vec4, Vec4, Vec4)> for Matrix4 {
    fn from(value: (Vec4, Vec4, Vec4, Vec4)) -> Self {
        Self(libobs_sys::matrix4 {
            x: unsafe { *value.0.as_ptr() },
            y: unsafe { *value.1.as_ptr() },
            z: unsafe { *value.2.as_ptr() },
            t: unsafe { *value.3.as_ptr() },
        })
    }
}

impl From<[Vec4; 4]> for Matrix4 {
    fn from(value: [Vec4; 4]) -> Self {
        Self(libobs_sys::matrix4 {
            x: unsafe { *value[0].as_ptr() },
            y: unsafe { *value[1].as_ptr() },
            z: unsafe { *value[2].as_ptr() },
            t: unsafe { *value[3].as_ptr() },
        })
    }
}
