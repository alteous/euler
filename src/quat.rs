use std::mem;

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
}

impl AsRef<[f32; 4]> for Quat {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Into<[f32; 4]> for Quat {
    fn into(self) -> [f32; 4] {
        [self.vector.x, self.vector.y, self.vector.z, self.scalar]
    }
}
