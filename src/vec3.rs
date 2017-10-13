use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::InnerSpace;
use {Vec2, Vec4};

/// 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,

    /// Z co-ordinate.
    pub z: f32,
}

impl Vec3 {
    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Vec3) -> f32 {
        let left = cgmath::Vector3::new(self.x, self.y, self.z);
        let right = cgmath::Vector3::new(other.x, other.y, other.z);
        left.dot(right)
    }

    /// Returns the length of the vector.
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns the cross product of two vectors.
    pub fn cross(self, other: Vec3) -> Vec3 {
        let left = cgmath::Vector3::new(self.x, self.y, self.z);
        let right = cgmath::Vector3::new(other.x, other.y, other.z);
        let result = left.cross(right);
        vec3!(result.x, result.y, result.z)
    }

    /// Returns a vector in the same direction but with unit magnitude.
    pub fn normalize(self) -> Vec3 {
        let n = cgmath::Vector3::new(self.x, self.y, self.z).normalize();
        vec3!(n.x, n.y, n.z)
    }

    /// Returns the XY components of the vector.
    pub fn xy(self) -> Vec2 {
        vec2!(self.x, self.y)
    }

    /// Returns the XZ components of the vector.
    pub fn xz(self) -> Vec2 {
        vec2!(self.x, self.z)
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Vec3 {
        vec3!(v[0], v[1], v[2])
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<Vec4> for Vec3 {
    fn from(vec4: Vec4) -> Vec3 {
        Vec3 { x: vec4.x, y: vec4.y, z: vec4.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        rhs.div(self)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
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

impl From<mint::Point3<f32>> for Vec3 {
    fn from(m: mint::Point3<f32>) -> Self {
        let m: [f32; 3] = m.into();
        Vec3::from(m)
    }
}

impl Into<mint::Point3<f32>> for Vec3 {
    fn into(self) -> mint::Point3<f32> {
        let m: [f32; 3] = self.into();
        mint::Point3::from(m)
    }
}

impl From<mint::Vector3<f32>> for Vec3 {
    fn from(m: mint::Vector3<f32>) -> Self {
        let m: [f32; 3] = m.into();
        Vec3::from(m)
    }
}

impl Into<mint::Vector3<f32>> for Vec3 {
    fn into(self) -> mint::Vector3<f32> {
        let m: [f32; 3] = self.into();
        mint::Vector3::from(m)
    }
}
