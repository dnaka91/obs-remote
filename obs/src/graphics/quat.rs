#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::{
    fmt::{self, Debug},
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Axisang, Matrix3, Matrix4, Vec3};

#[derive(Clone, Copy, Default)]
pub struct Quat(libobs_sys::quat);

impl Quat {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(libobs_sys::quat {
            __bindgen_anon_1: libobs_sys::quat__bindgen_ty_1 {
                m: unsafe { mem::transmute(_mm_set_ps(x, y, z, w)) },
            },
        })
    }

    #[inline]
    fn new_ptr(ptr: [f32; 4]) -> Self {
        Self(libobs_sys::quat {
            __bindgen_anon_1: libobs_sys::quat__bindgen_ty_1 { ptr },
        })
    }

    #[inline]
    fn new_m(m: __m128) -> Self {
        Self(libobs_sys::quat {
            __bindgen_anon_1: libobs_sys::quat__bindgen_ty_1 {
                m: unsafe { mem::transmute(m) },
            },
        })
    }

    pub(crate) fn as_ptr(&self) -> *const libobs_sys::quat {
        &self.0 as _
    }

    pub fn as_ptr_mut(&mut self) -> *mut libobs_sys::quat {
        &mut self.0 as _
    }

    #[inline]
    fn set_m(&mut self, m: __m128) {
        self.0.__bindgen_anon_1.m = unsafe { mem::transmute(m) };
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

    #[inline]
    pub fn identity() -> Self {
        let mut q = Self::default();
        q.0.__bindgen_anon_1.m = unsafe { mem::transmute(_mm_setzero_ps()) };
        q.0.__bindgen_anon_1.__bindgen_anon_1.w = 1.0;
        q
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        let m = unsafe {
            _mm_mul_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        };
        let m = unsafe { _mm_add_ps(_mm_movehl_ps(m, m), m) };
        let m = unsafe { _mm_add_ps(_mm_shuffle_ps(m, m, 0x55), m) };

        Self::new_m(m).x()
    }

    #[inline]
    pub fn inv(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z(), self.w())
    }

    #[inline]
    pub fn len(self) -> f32 {
        let dot = self.dot(self);
        if dot > 0.0 {
            dot.sqrt()
        } else {
            0.0
        }
    }

    #[inline]
    pub fn dist(self, rhs: Self) -> f32 {
        let temp = self - rhs;
        let dot = temp.dot(temp);
        if dot > 0.0 {
            dot.sqrt()
        } else {
            0.0
        }
    }

    #[inline]
    pub fn norm(self) -> Self {
        let dot = self.dot(self);
        Self::new_m(if dot > 0.0 {
            unsafe {
                _mm_mul_ps(
                    mem::transmute(self.0.__bindgen_anon_1.m),
                    _mm_set1_ps(1.0 / dot.sqrt()),
                )
            }
        } else {
            unsafe { _mm_setzero_ps() }
        })
    }

    #[inline]
    pub fn close(self, rhs: Self, epsilon: f32) -> bool {
        let test = self - rhs;
        test.x() < epsilon && test.y() < epsilon && test.z() < epsilon && test.w() < epsilon
    }

    pub fn get_dir(self) -> Vec3 {
        let mut dst = Vec3::default();
        unsafe { libobs_sys::quat_get_dir(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    pub fn set_look_dir(mut self, dir: Vec3) -> Self {
        unsafe { libobs_sys::quat_set_look_dir(self.as_ptr_mut(), dir.as_ptr()) };
        self
    }

    pub fn log(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_log(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    pub fn exp(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_exp(dst.as_ptr_mut(), self.as_ptr()) };
        dst
    }

    pub fn interpolate(self, q2: Self, t: f32) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_interpolate(dst.as_ptr_mut(), self.as_ptr(), q2.as_ptr(), t) };
        dst
    }

    pub fn get_tangent(self, prev: Self, next: Self) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::quat_get_tangent(
                dst.as_ptr_mut(),
                prev.as_ptr(),
                self.as_ptr(),
                next.as_ptr(),
            )
        };
        dst
    }

    pub fn interpolate_cubic(self, q2: Self, m1: Self, m2: Self, t: f32) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::quat_interpolate_cubic(
                dst.as_ptr_mut(),
                self.as_ptr(),
                q2.as_ptr(),
                m1.as_ptr(),
                m2.as_ptr(),
                t,
            )
        };
        dst
    }
}

impl Debug for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Quat")
            .field(&self.x())
            .field(&self.y())
            .field(&self.z())
            .field(&self.w())
            .finish()
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_add_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl AddAssign for Quat {
    fn add_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_add_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Sub for Quat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_sub_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl SubAssign for Quat {
    fn sub_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_sub_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_mul(dst.as_ptr_mut(), self.as_ptr(), rhs.as_ptr()) };
        dst
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_mul(dst.as_ptr_mut(), self.as_ptr(), rhs.as_ptr()) };
        self.set_m(unsafe { mem::transmute(dst.0.__bindgen_anon_1.m) });
    }
}

impl Add<f32> for Quat {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_add_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl AddAssign<f32> for Quat {
    fn add_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_add_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Sub<f32> for Quat {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_sub_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl SubAssign<f32> for Quat {
    fn sub_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_sub_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_mul_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl MulAssign<f32> for Quat {
    fn mul_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_mul_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_div_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl DivAssign<f32> for Quat {
    fn div_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_div_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Neg for Quat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl From<Axisang> for Quat {
    fn from(value: Axisang) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_from_axisang(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Matrix3> for Quat {
    fn from(value: Matrix3) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_from_matrix3(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<Matrix4> for Quat {
    fn from(value: Matrix4) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::quat_from_matrix4(dst.as_ptr_mut(), value.as_ptr()) };
        dst
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<[f32; 4]> for Quat {
    fn from(value: [f32; 4]) -> Self {
        Self::new_ptr(value)
    }
}
