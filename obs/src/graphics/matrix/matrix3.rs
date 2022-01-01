use std::{
    fmt::{self, Debug},
    ops::Mul,
};

use super::Matrix4;
use crate::graphics::{Axisang, Plane, Quat, Vec3};

#[derive(Clone, Copy, Default)]
pub struct Matrix3(libobs_sys::matrix3);

impl Matrix3 {
    pub(crate) fn as_ptr(&self) -> *const libobs_sys::matrix3 {
        &self.0 as _
    }

    fn as_ptr_mut(&mut self) -> *mut libobs_sys::matrix3 {
        &mut self.0 as _
    }

    #[inline]
    pub fn identity() -> Self {
        let mut new = Self::default();
        new.0.x.__bindgen_anon_1.__bindgen_anon_1.x = 1.0;
        new.0.y.__bindgen_anon_1.__bindgen_anon_1.y = 1.0;
        new.0.z.__bindgen_anon_1.__bindgen_anon_1.z = 1.0;
        new
    }

    #[inline]
    #[must_use]
    pub fn translate(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        dst.0.t = unsafe { *(Vec3::from_raw(self.0.t) - v).as_ptr() };
        dst
    }

    #[must_use]
    pub fn rotate(self, q: Quat) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_rotate(dst.as_ptr_mut(), self.as_ptr(), q.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn rotate_aa(self, aa: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_rotate_aa(dst.as_ptr_mut(), self.as_ptr(), aa.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn scale(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_scale(dst.as_ptr_mut(), self.as_ptr(), v.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn transpose(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_transpose(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn inv(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_inv(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn mirror(self, p: Plane) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_mirror(dst.as_ptr_mut(), self.as_ptr(), p.as_ptr()) };
        dst
    }

    #[must_use]
    pub fn mirrorv(self, v: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_mirrorv(dst.as_ptr_mut(), self.as_ptr(), v.as_ptr()) };
        dst
    }

    #[inline]
    #[must_use]
    pub fn translate3f(self, x: f32, y: f32, z: f32) -> Self {
        self.translate(Vec3::new(x, y, z))
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

impl Debug for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matrix3")
            .field(&Vec3::from_raw(self.0.x))
            .field(&Vec3::from_raw(self.0.y))
            .field(&Vec3::from_raw(self.0.z))
            .field(&Vec3::from_raw(self.0.t))
            .finish()
    }
}

impl From<Quat> for Matrix3 {
    fn from(value: Quat) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_from_quat(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Axisang> for Matrix3 {
    fn from(value: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_from_axisang(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Matrix4> for Matrix3 {
    fn from(value: Matrix4) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_from_matrix4(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut dst = Self::default();
        unsafe { libobs_sys::matrix3_mul(dst.as_ptr_mut(), self.as_ptr(), rhs.as_ptr()) };
        dst
    }
}

impl From<(Vec3, Vec3, Vec3, Vec3)> for Matrix3 {
    fn from(value: (Vec3, Vec3, Vec3, Vec3)) -> Self {
        Self(libobs_sys::matrix3 {
            x: unsafe { *value.0.as_ptr() },
            y: unsafe { *value.1.as_ptr() },
            z: unsafe { *value.2.as_ptr() },
            t: unsafe { *value.3.as_ptr() },
        })
    }
}

impl From<[Vec3; 4]> for Matrix3 {
    fn from(value: [Vec3; 4]) -> Self {
        Self(libobs_sys::matrix3 {
            x: unsafe { *value[0].as_ptr() },
            y: unsafe { *value[1].as_ptr() },
            z: unsafe { *value[2].as_ptr() },
            t: unsafe { *value[3].as_ptr() },
        })
    }
}
