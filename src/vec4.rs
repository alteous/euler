use cgmath;
use std::mem;

use approx::ApproxEq;
use cgmath::InnerSpace;

/// Homogeneous 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,

    /// Z co-ordinate.
    pub z: f32,

    /// Homogeneous W co-ordinate.
    pub w: f32,
}

impl Vec4 {
    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Vec4) -> f32 {
        let left = cgmath::Vector4::new(self.x, self.y, self.z, self.w);
        let right = cgmath::Vector4::new(other.x, other.y, other.z, other.w);
        left.dot(right)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Into<[f32; 4]> for Vec4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl ApproxEq for Vec4 {
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
            &&
        <f32 as ApproxEq>::relative_eq(&self.w, &other.w, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as ApproxEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
    }
}
