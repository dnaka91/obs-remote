#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::{
    fmt::{self, Debug},
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Max, Min, VecX};
use crate::graphics::{Matrix3, Matrix4, Plane};

#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[derive(Copy, Clone, Default)]
pub struct Vec3(libobs_sys::vec3);

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(libobs_sys::vec3 {
            __bindgen_anon_1: libobs_sys::vec3__bindgen_ty_1 {
                m: unsafe { mem::transmute(_mm_set_ps(0.0, z, y, x)) },
            },
        })
    }

    #[inline]
    fn new_ptr(ptr: [f32; 3]) -> Self {
        Self(libobs_sys::vec3 {
            __bindgen_anon_1: libobs_sys::vec3__bindgen_ty_1 {
                m: unsafe { mem::transmute(_mm_set_ps(0.0, ptr[2], ptr[1], ptr[0])) },
            },
        })
    }

    #[inline]
    fn new_m(m: __m128) -> Self {
        let mut new = Self(libobs_sys::vec3 {
            __bindgen_anon_1: libobs_sys::vec3__bindgen_ty_1 {
                m: unsafe { mem::transmute(m) },
            },
        });
        new.0.__bindgen_anon_1.__bindgen_anon_1.w = 0.0;
        new
    }

    #[inline]
    fn set_m(&mut self, m: __m128) {
        self.0.__bindgen_anon_1.m = unsafe { mem::transmute(m) };
    }

    pub(crate) fn from_raw(raw: libobs_sys::vec3) -> Self {
        Self(raw)
    }

    pub(crate) fn as_ptr(&self) -> *const libobs_sys::vec3 {
        &self.0 as _
    }

    pub(crate) fn as_ptr_mut(&mut self) -> *mut libobs_sys::vec3 {
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
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        let s1v1 = unsafe {
            _mm_shuffle_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(self.0.__bindgen_anon_1.m),
                _MM_SHUFFLE(3, 0, 2, 1),
            )
        };
        let s1v2 = unsafe {
            _mm_shuffle_ps(
                mem::transmute(rhs.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
                _MM_SHUFFLE(3, 1, 0, 2),
            )
        };
        let s2v1 = unsafe {
            _mm_shuffle_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(self.0.__bindgen_anon_1.m),
                _MM_SHUFFLE(3, 1, 0, 2),
            )
        };
        let s2v2 = unsafe {
            _mm_shuffle_ps(
                mem::transmute(rhs.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
                _MM_SHUFFLE(3, 0, 2, 1),
            )
        };
        Self::new_m(unsafe { _mm_sub_ps(_mm_mul_ps(s1v1, s1v2), _mm_mul_ps(s2v1, s2v2)) })
    }

    pub fn plane_dist(self, plane: Plane) -> f32 {
        unsafe { libobs_sys::vec3_plane_dist(self.as_ptr(), plane.as_ptr()) }
    }

    #[must_use]
    pub fn transform(self, m: Matrix4) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_transform((&mut dst.0) as _, self.as_ptr(), m.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn rotate(self, m: Matrix3) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_rotate((&mut dst.0) as _, self.as_ptr(), m.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn transform3x4(self, m: Matrix3) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_transform3x4((&mut dst.0) as _, self.as_ptr(), m.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn mirror(self, p: Plane) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_mirror((&mut dst.0) as _, self.as_ptr(), p.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn mirrorv(self, vec: Vec3) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_mirrorv((&mut dst.0) as _, self.as_ptr(), vec.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn rand(self, positive_only: bool) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::vec3_rand((&mut dst.0) as _, if positive_only { 1 } else { 0 });
        }
        dst
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec3")
            .field(&self.x())
            .field(&self.y())
            .field(&self.z())
            .finish()
    }
}

impl Add for Vec3 {
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_add_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Sub for Vec3 {
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

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_sub_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_mul_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_mul_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_div_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_div_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_add_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_add_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_sub_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_sub_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_mul_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_mul_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_div_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_div_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        });
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Min for Vec3 {
    type Output = Self;

    fn min(self, rhs: Self) -> Self {
        Self::new_m(unsafe {
            _mm_min_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl Max for Vec3 {
    type Output = Self;

    fn max(self, rhs: Self) -> Self {
        Self::new_m(unsafe {
            _mm_max_ps(
                mem::transmute(self.0.__bindgen_anon_1.m),
                mem::transmute(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl Min<f32> for Vec3 {
    type Output = Self;

    fn min(self, rhs: f32) -> Self {
        Self::new_m(unsafe {
            _mm_min_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl Max<f32> for Vec3 {
    type Output = Self;

    fn max(self, rhs: f32) -> Self {
        Self::new_m(unsafe {
            _mm_max_ps(mem::transmute(self.0.__bindgen_anon_1.m), _mm_set1_ps(rhs))
        })
    }
}

impl VecX<f32> for Vec3 {
    fn zero(&mut self) {
        self.set_m(unsafe { mem::transmute(_mm_setzero_ps()) });
    }

    fn dot(self, rhs: Self) -> f32 {
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

    fn len(self) -> f32 {
        let dot = self.dot(self);
        if dot > 0.0 {
            dot.sqrt()
        } else {
            0.0
        }
    }

    fn dist(self, rhs: Self) -> f32 {
        let temp = self - rhs;
        let dot = temp.dot(temp);
        if dot > 0.0 {
            dot.sqrt()
        } else {
            0.0
        }
    }

    fn norm(self) -> Self {
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

    fn close(self, rhs: Self, epsilon: f32) -> bool {
        let test = self - rhs;
        test.x() < epsilon && test.y() < epsilon && test.z() < epsilon
    }

    fn abs(self) -> Self {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    fn floor(self) -> Self {
        Self::new(self.x().floor(), self.y().floor(), self.z().floor())
    }

    fn ceil(self) -> Self {
        Self::new(self.x().ceil(), self.y().ceil(), self.z().ceil())
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self::new_ptr(value)
    }
}
