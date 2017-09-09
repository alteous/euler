use cgmath;
use std::mem;

use {Quat, Mat4, Vec3};

/// Translation + Rotation + Non-uniform Scale transform in 3D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3 {
    /// Translation vector.
    pub translation: Vec3,

    /// Rotation quaternion.
    pub rotation: Quat,

    /// *Non-uniform* scaling.
    pub scale: Vec3,
}

impl Transform3 {
    /// Returns the identity transform.
    pub fn identity() -> Self {
        Transform3 {
            translation: vec3!(),
            rotation: Quat::identity(),
            scale: vec3!(1.0),
        }
    }

    /// Returns the equivalent matrix representation for this transform.
    pub fn matrix(&self) -> Mat4 {
        let t = cgmath::Matrix4::from_translation(
            cgmath::Vector3::new(
                self.translation.x,
                self.translation.y,
                self.translation.z,
            ),
        );
        let r = cgmath::Matrix4::from(
            cgmath::Quaternion::new(
                self.rotation.vector.x,
                self.rotation.vector.y,
                self.rotation.vector.z,
                self.rotation.scalar,
            ),
        );
        let s = cgmath::Matrix4::from_nonuniform_scale(
            self.scale.x,
            self.scale.y,
            self.scale.z,
        );
        let matrix: [[f32; 4]; 4] = (t * r * s).into();
        unsafe {
            mem::transmute(matrix)
        }
    }
}
