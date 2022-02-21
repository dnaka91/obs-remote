use std::fmt::{self, Debug};

use super::{
    matrix::{Matrix3, Matrix4},
    vec::{Vec3, VecX},
};

#[derive(Copy, Clone, Default)]
pub struct Plane(libobs_sys::plane);

impl Plane {
    pub(crate) fn as_ptr(&self) -> *const libobs_sys::plane {
        &self.0 as _
    }

    fn as_ptr_mut(&mut self) -> *mut libobs_sys::plane {
        &mut self.0 as _
    }

    #[inline]
    pub fn new(dir: Vec3, dist: f32) -> Self {
        Self(libobs_sys::plane {
            dir: unsafe { *dir.as_ptr() },
            dist,
        })
    }

    #[inline]
    pub fn newf(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self(libobs_sys::plane {
            dir: unsafe { *Vec3::new(a, b, c).as_ptr() },
            dist: d,
        })
    }

    #[inline]
    pub fn setf(&mut self, a: f32, b: f32, c: f32, d: f32) {
        self.0.dir = unsafe { *Vec3::new(a, b, c).as_ptr() };
        self.0.dist = d;
    }

    #[must_use]
    pub fn transform(self, m: Matrix4) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::plane_transform(dst.as_ptr_mut(), self.as_ptr(), m.as_ptr());
        }
        dst
    }

    #[must_use]
    pub fn transform3x4(self, m: Matrix3) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::plane_transform3x4(dst.as_ptr_mut(), self.as_ptr(), m.as_ptr());
        }
        dst
    }

    pub fn intersection_ray(self, orig: Vec3, dir: Vec3, t: &mut f32) -> bool {
        unsafe {
            libobs_sys::plane_intersection_ray(self.as_ptr(), orig.as_ptr(), dir.as_ptr(), t as _)
        }
    }

    pub fn intersection_line(self, v1: Vec3, v2: Vec3, t: &mut f32) -> bool {
        unsafe {
            libobs_sys::plane_intersection_line(self.as_ptr(), v1.as_ptr(), v2.as_ptr(), t as _)
        }
    }

    pub fn tri_inside(self, v1: Vec3, v2: Vec3, v3: Vec3, precision: f32) -> bool {
        unsafe {
            libobs_sys::plane_tri_inside(
                self.as_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                precision,
            )
        }
    }

    pub fn line_inside(self, v1: Vec3, v2: Vec3, precision: f32) -> bool {
        unsafe { libobs_sys::plane_line_inside(self.as_ptr(), v1.as_ptr(), v2.as_ptr(), precision) }
    }

    #[inline]
    pub fn close(self, p2: Self, precision: f32) -> bool {
        let p1_dir = Vec3::from_raw(self.0.dir);
        let p2_dir = Vec3::from_raw(p2.0.dir);

        p1_dir.close(p2_dir, precision) && close_float(self.0.dist, p2.0.dist, precision)
    }

    #[inline]
    pub fn coplanar(self, p2: Self, precision: f32) -> bool {
        let p1_dir = Vec3::from_raw(self.0.dir);
        let p2_dir = Vec3::from_raw(p2.0.dir);
        let cos_angle = p1_dir.dot(p2_dir);

        if close_float(cos_angle, 1.0, precision) {
            close_float(self.0.dist, p2.0.dist, precision)
        } else if close_float(cos_angle, -1.0, precision) {
            close_float(-self.0.dist, p2.0.dist, precision)
        } else {
            false
        }
    }
}

impl Debug for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Plane")
            .field(&Vec3::from_raw(self.0.dir))
            .field(&self.0.dist)
            .finish()
    }
}

impl From<(Vec3, Vec3, Vec3)> for Plane {
    fn from(value: (Vec3, Vec3, Vec3)) -> Self {
        let mut dst = Self::default();
        unsafe {
            libobs_sys::plane_from_tri(
                dst.as_ptr_mut(),
                value.0.as_ptr(),
                value.1.as_ptr(),
                value.2.as_ptr(),
            )
        };
        dst
    }
}

fn close_float(f1: f32, f2: f32, precision: f32) -> bool {
    (f1 - f2).abs() <= precision
}
