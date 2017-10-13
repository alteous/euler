use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::InnerSpace;

use Vec3;

/// 4D vector.
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

    /// Returns the length of the vector.
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns the XYZ components of the vector.
    pub fn xyz(self) -> Vec3 {
        vec3!(self.x, self.y, self.z)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(v: [f32; 4]) -> Vec4 {
        vec4!(v[0], v[1], v[2], v[3])
    }
}

impl Into<[f32; 4]> for Vec4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f32) -> Self::Output {
        Vec4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::Div<Vec4> for f32 {
    type Output = Vec4;
    fn div(self, rhs: Vec4) -> Self::Output {
        rhs.div(self)
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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

impl From<mint::Vector4<f32>> for Vec4 {
    fn from(m: mint::Vector4<f32>) -> Self {
        let m: [f32; 4] = m.into();
        Vec4::from(m)
    }
}

impl Into<mint::Vector4<f32>> for Vec4 {
    fn into(self) -> mint::Vector4<f32> {
        let m: [f32; 4] = self.into();
        mint::Vector4::from(m)
    }
}
