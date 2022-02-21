use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Max, Min, VecX};

#[derive(Copy, Clone, Default)]
pub struct Vec2(libobs_sys::vec2);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(libobs_sys::vec2 {
            __bindgen_anon_1: libobs_sys::vec2__bindgen_ty_1 {
                __bindgen_anon_1: libobs_sys::vec2__bindgen_ty_1__bindgen_ty_1 { x, y },
            },
        })
    }

    fn new_ptr(ptr: [f32; 2]) -> Self {
        Self(libobs_sys::vec2 {
            __bindgen_anon_1: libobs_sys::vec2__bindgen_ty_1 { ptr },
        })
    }

    #[inline]
    fn set_ptr(&mut self, ptr: [f32; 2]) {
        self.0.__bindgen_anon_1.ptr = ptr;
    }

    pub(crate) fn as_ptr(&self) -> *const libobs_sys::vec2 {
        &self.0 as _
    }

    pub(crate) fn as_ptr_mut(&mut self) -> *mut libobs_sys::vec2 {
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
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec2")
            .field(&self.x())
            .field(&self.y())
            .finish()
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.set_ptr([self.x() + rhs.x(), self.y() + rhs.y()]);
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.set_ptr([self.x() - rhs.x(), self.y() - rhs.y()]);
    }
}

impl Mul for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y())
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.set_ptr([self.x() * rhs.x(), self.y() * rhs.y()]);
    }
}

impl Div for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y())
    }
}

impl DivAssign for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.set_ptr([self.x() / rhs.x(), self.y() / rhs.y()]);
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x() + rhs, self.y() + rhs)
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.set_ptr([self.x() + rhs, self.y() + rhs]);
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.x() - rhs, self.y() - rhs)
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.set_ptr([self.x() - rhs, self.y() - rhs]);
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.set_ptr([self.x() * rhs, self.y() * rhs]);
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.set_ptr([self.x() / rhs, self.y() / rhs]);
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y())
    }
}

impl Min for Vec2 {
    type Output = Self;

    #[inline]
    fn min(self, rhs: Self) -> Self::Output {
        Self::new(self.x().min(rhs.x()), self.y().min(rhs.y()))
    }
}

impl Max for Vec2 {
    type Output = Self;

    #[inline]
    fn max(self, rhs: Self) -> Self::Output {
        Self::new(self.x().max(rhs.x()), self.y().max(rhs.y()))
    }
}

impl Min<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn min(self, rhs: f32) -> Self::Output {
        Self::new(self.x().min(rhs), self.y().min(rhs))
    }
}

impl Max<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn max(self, rhs: f32) -> Self::Output {
        Self::new(self.x().max(rhs), self.y().max(rhs))
    }
}

impl VecX<f32> for Vec2 {
    #[inline]
    fn zero(&mut self) {
        self.set_ptr([0.0, 0.0]);
    }

    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    #[inline]
    fn len(self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }

    #[inline]
    fn dist(self, rhs: Self) -> f32 {
        (self - rhs).len()
    }

    #[inline]
    fn norm(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::vec2_norm((&mut dst.0) as _, self.as_ptr()) };
        dst
    }

    #[inline]
    fn close(self, rhs: Self, epsilon: f32) -> bool {
        unsafe { libobs_sys::vec2_close(self.as_ptr(), rhs.as_ptr(), epsilon) > 0 }
    }

    #[inline]
    fn abs(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::vec2_abs((&mut dst.0) as _, self.as_ptr()) };
        dst
    }

    #[inline]
    fn floor(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::vec2_floor((&mut dst.0) as _, self.as_ptr()) };
        dst
    }

    #[inline]
    fn ceil(self) -> Self {
        let mut dst = Self::default();
        unsafe { libobs_sys::vec2_ceil((&mut dst.0) as _, self.as_ptr()) };
        dst
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Self::new_ptr(value)
    }
}
