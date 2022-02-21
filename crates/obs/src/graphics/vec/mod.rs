use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub use self::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

mod vec2;
mod vec3;
mod vec4;

pub trait Min<Rhs = Self> {
    type Output;

    fn min(self, rhs: Rhs) -> Self::Output;
}

pub trait Max<Rhs = Self> {
    type Output;

    fn max(self, rhs: Rhs) -> Self::Output;
}

#[allow(clippy::len_without_is_empty)]
pub trait VecX<T>:
    Copy
    + Default
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Add<T>
    + AddAssign<T>
    + Sub<T>
    + SubAssign<T>
    + Mul<T>
    + MulAssign<T>
    + Div<T>
    + DivAssign<T>
    + Neg
    + Min
    + Max
    + Min<T>
    + Max<T>
{
    fn zero(&mut self);
    fn dot(self, rhs: Self) -> T;
    fn len(self) -> T;
    fn dist(self, rhs: Self) -> T;
    #[must_use]
    fn norm(self) -> Self;
    fn close(self, rhs: Self, epsilon: T) -> bool;
    #[must_use]
    fn abs(self) -> Self;
    #[must_use]
    fn floor(self) -> Self;
    #[must_use]
    fn ceil(self) -> Self;
}
