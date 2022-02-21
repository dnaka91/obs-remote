pub use axisang::Axisang;
pub use matrix::{Matrix3, Matrix4};
pub use plane::Plane;
pub use quat::Quat;
pub use vec::{Vec2, Vec3, Vec4, VecX};

mod axisang;
mod matrix;
mod plane;
mod quat;
mod vec;

pub fn enter() {
    unsafe { libobs_sys::obs_enter_graphics() };
}

pub fn leave() {
    unsafe { libobs_sys::obs_leave_graphics() };
}

pub fn scoped<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    enter();
    let value = f();
    leave();

    value
}
