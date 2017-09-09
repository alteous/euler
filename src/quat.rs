use cgmath;
use std::mem;

use cgmath::Rotation3;
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

    /// Returns the quaternion representation of a rotation around the X axis.
    pub fn xrot(angle: f32) -> Self {
        cgmath::Quaternion::from_angle_x(cgmath::Rad(angle)).into()
    }

    /// Returns the quaternion representation of a rotation around the Y axis.
    pub fn yrot(angle: f32) -> Self {
        cgmath::Quaternion::from_angle_y(cgmath::Rad(angle)).into()
    }

    /// Returns the quaternion representation of a rotation around the Z axis.
    pub fn zrot(angle: f32) -> Self {
        cgmath::Quaternion::from_angle_z(cgmath::Rad(angle)).into()
    }
}

impl AsRef<[f32; 4]> for Quat {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<cgmath::Quaternion<f32>> for Quat {
    fn from(q: cgmath::Quaternion<f32>) -> Self {
        quat!(q.s; q.v.x, q.v.y, q.v.z)
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

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Quat;

    #[cfg(feature = "mint-support")]
    impl From<mint::Quaternion<f32>> for Quat {
        fn from(q: mint::Quaternion<f32>) -> Self {
            quat!(q.s; q.v.x, q.v.y, q.v.z)
        }
    }

    #[cfg(feature = "mint-support")]
    impl Into<mint::Quaternion<f32>> for Quat {
        fn into(self) -> mint::Quaternion<f32> {
            mint::Quaternion {
                s: self.scalar,
                v: self.vector.into(),
            }
        }
    }
}
