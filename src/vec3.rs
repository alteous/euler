use cgmath;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::InnerSpace;
use Vec4;

/// 3D vector.
///
/// # Note
///
/// The representation is a homogeneous 3D vector with a fixed `w` value of 0.0.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,

    /// Z co-ordinate.
    pub z: f32,

    /// Homogeneous W co-ordinate - always `0.0` for 3D vectors.
    #[doc(hidden)]
    pub w: f32,
}

impl Vec3 {
    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Vec3) -> f32 {
        let left = cgmath::Vector3::new(self.x, self.y, self.z);
        let right = cgmath::Vector3::new(other.x, other.y, other.z);
        left.dot(right)
    }

    /// Returns the cross product of two vectors.
    pub fn cross(self, other: Vec3) -> Vec3 {
        let left = cgmath::Vector3::new(self.x, self.y, self.z);
        let right = cgmath::Vector3::new(other.x, other.y, other.z);
        let result = left.cross(right);
        vec3!(result.x, result.y, result.z)
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<Vec4> for Vec3 {
    fn from(vec4: Vec4) -> Vec3 {
        Vec3 { x: vec4.x, y: vec4.y, z: vec4.z, w: 0.0 }
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: 0.0,
        }
    }
}
    
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: 0.0,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 0.0,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 0.0,
        }
    }
}

impl ApproxEq for Vec3 {
    type Epsilon = <f32 as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        <f32 as ApproxEq>::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        <f32 as ApproxEq>::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        <f32 as ApproxEq>::default_max_ulps()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        <f32 as ApproxEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            &&
        <f32 as ApproxEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            &&
        <f32 as ApproxEq>::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as ApproxEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}
