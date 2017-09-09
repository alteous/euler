use cgmath;
use mint;
use std::mem;

use cgmath::{InnerSpace, Rotation3};
use Vec3;

/// Quaternion in the order `[x, y, z, w]`, where `w` is the scalar.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Quat {
    /// Vector part.
    pub vector: Vec3,

    /// Scalar part.
    pub scalar: f32,
}

impl Quat {
    /// Returns the identity quaternion.
    pub fn identity() -> Self {
        Quat {
            scalar: 1.0,
            vector: vec3!(),
        }
    }

    /// Returns a quaternion representing a rotation by `r` radians about `axis`.
    pub fn rotation_about_axis(axis: Vec3, r: f32) -> Self {
        let q = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::new(axis.x, axis.y, axis.z).normalize(),
            cgmath::Rad(r),
        );
        Quat {
            vector: vec3!(q.v.x, q.v.y, q.v.z),
            scalar: q.s,
        }
    }
}

impl AsRef<[f32; 4]> for Quat {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 4]> for Quat {
    fn from(q: [f32; 4]) -> Self {
        Quat {
            vector: vec3!(q[0], q[1], q[2]),
            scalar: q[3],
        }
    }
}

impl Into<[f32; 4]> for Quat {
    fn into(self) -> [f32; 4] {
        [self.vector.x, self.vector.y, self.vector.z, self.scalar]
    }
}

impl From<mint::Quaternion<f32>> for Quat {
    fn from(q: mint::Quaternion<f32>) -> Self {
        Quat {
            vector: vec3!(q.v.x, q.v.y, q.v.z),
            scalar: q.s,
        }
    }
}

impl Into<mint::Quaternion<f32>> for Quat {
    fn into(self) -> mint::Quaternion<f32> {
        mint::Quaternion {
            s: self.scalar,
            v: self.vector.into(),
        }
    }
}
