#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Max, Min, VecX};
use crate::gs;

#[derive(Copy, Clone, Default)]
pub struct Vec4(libobs_sys::vec4);

impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(libobs_sys::vec4 {
            __bindgen_anon_1: libobs_sys::vec4__bindgen_ty_1 {
                m: zerocopy::transmute!(unsafe { _mm_set_ps(w, z, y, x) }),
            },
        })
    }

    #[inline]
    fn new_ptr(ptr: [f32; 4]) -> Self {
        Self(libobs_sys::vec4 {
            __bindgen_anon_1: libobs_sys::vec4__bindgen_ty_1 { ptr },
        })
    }

    #[inline]
    fn new_m(m: __m128) -> Self {
        Self(libobs_sys::vec4 {
            __bindgen_anon_1: libobs_sys::vec4__bindgen_ty_1 {
                m: zerocopy::transmute!(m),
            },
        })
    }

    #[inline]
    fn set_m(&mut self, m: __m128) {
        self.0.__bindgen_anon_1.m = zerocopy::transmute!(m);
    }

    pub(crate) fn from_raw(raw: libobs_sys::vec4) -> Self {
        Self(raw)
    }

    pub(crate) fn as_ptr(&self) -> *const libobs_sys::vec4 {
        &self.0 as _
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
    pub fn to_rgba(self) -> u32 {
        let f = unsafe { self.0.__bindgen_anon_1.ptr };
        u32::from_be_bytes(gs::float4_to_u8x4(f))
    }

    #[inline]
    pub fn to_bgra(self) -> u32 {
        let f = unsafe { self.0.__bindgen_anon_1.ptr };
        let mut u = gs::float4_to_u8x4(f);
        u.swap(0, 2);
        u32::from_be_bytes(u)
    }

    #[inline]
    pub fn from_rgba(rgba: u32) -> Self {
        Self::new_ptr(gs::u8x4_to_float4(rgba.to_be_bytes()))
    }

    #[inline]
    pub fn from_bgra(bgra: u32) -> Self {
        let mut u = bgra.to_be_bytes();
        u.swap(0, 2);
        Self::new_ptr(gs::u8x4_to_float4(u))
    }

    #[inline]
    pub fn from_rgba_srgb(rgba: u32) -> Self {
        let v = Self::from_rgba(rgba);
        let mut ptr = [v.x(), v.y(), v.z()];
        gs::float3_srgb_nonlinear_to_linear(&mut ptr);
        Self::new(ptr[0], ptr[1], ptr[2], v.w())
    }
}

impl Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec4")
            .field(&self.x())
            .field(&self.y())
            .field(&self.z())
            .field(&self.w())
            .finish()
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_add_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_add_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_sub_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_sub_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_mul_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_mul_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new_m(unsafe {
            _mm_div_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.set_m(unsafe {
            _mm_div_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        });
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_add_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_add_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        });
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_sub_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_sub_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        });
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_mul_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_mul_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        });
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new_m(unsafe {
            _mm_div_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self.set_m(unsafe {
            _mm_div_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        });
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl Min for Vec4 {
    type Output = Self;

    fn min(self, rhs: Self) -> Self {
        Self::new_m(unsafe {
            _mm_min_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl Max for Vec4 {
    type Output = Self;

    fn max(self, rhs: Self) -> Self {
        Self::new_m(unsafe {
            _mm_max_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
            )
        })
    }
}

impl Min<f32> for Vec4 {
    type Output = Self;

    fn min(self, rhs: f32) -> Self {
        Self::new_m(unsafe {
            _mm_min_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl Max<f32> for Vec4 {
    type Output = Self;

    fn max(self, rhs: f32) -> Self {
        Self::new_m(unsafe {
            _mm_max_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                _mm_set1_ps(rhs),
            )
        })
    }
}

impl VecX<f32> for Vec4 {
    fn zero(&mut self) {
        self.set_m(unsafe { _mm_setzero_ps() });
    }

    fn dot(self, rhs: Self) -> f32 {
        let m = unsafe {
            _mm_mul_ps(
                zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                zerocopy::transmute!(rhs.0.__bindgen_anon_1.m),
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
                    zerocopy::transmute!(self.0.__bindgen_anon_1.m),
                    _mm_set1_ps(1.0 / dot.sqrt()),
                )
            }
        } else {
            unsafe { _mm_setzero_ps() }
        })
    }

    fn close(self, rhs: Self, epsilon: f32) -> bool {
        let test = self - rhs;
        test.x() < epsilon && test.y() < epsilon && test.z() < epsilon && test.w() < epsilon
    }

    fn abs(self) -> Self {
        Self::new(
            self.x().abs(),
            self.y().abs(),
            self.z().abs(),
            self.w().abs(),
        )
    }

    fn floor(self) -> Self {
        Self::new(
            self.x().floor(),
            self.y().floor(),
            self.z().floor(),
            self.w().floor(),
        )
    }

    fn ceil(self) -> Self {
        Self::new(
            self.x().ceil(),
            self.y().ceil(),
            self.z().ceil(),
            self.w().ceil(),
        )
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self::new_ptr(value)
    }
}
