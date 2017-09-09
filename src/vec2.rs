use cgmath;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::InnerSpace;
use {Vec3, Vec4};

/// 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,
}

impl Vec2 {
    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Vec2) -> f32 {
        let left = cgmath::Vector2::new(self.x, self.y);
        let right = cgmath::Vector2::new(other.x, other.y);
        left.dot(right)
    }

    /// Returns a vector in the same direction but with unit magnitude.
    pub fn normalize(self) -> Vec2 {
        let n: [f32; 2] = cgmath::Vector2::new(self.x, self.y).normalize().into();
        n.into()
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    fn as_ref(&self) -> &[f32; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Vec2 {
        vec2!(v[0], v[1])
    }
}

impl Into<[f32; 2]> for Vec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<Vec3> for Vec2 {
    fn from(vec3: Vec3) -> Vec2 {
        Vec2 { x: vec3.x, y: vec3.y }
    }
}

impl From<Vec4> for Vec2 {
    fn from(vec4: Vec4) -> Vec2 {
        Vec2 { x: vec4.x, y: vec4.y }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ApproxEq for Vec2 {
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
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as ApproxEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            &&
        <f32 as ApproxEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Vec2;

    impl From<mint::Point2<f32>> for Vec2 {
        fn from(m: mint::Point2<f32>) -> Self {
            let m: [f32; 2] = m.into();
            Vec2::from(m)
        }
    }

    impl Into<mint::Point2<f32>> for Vec2 {
        fn into(self) -> mint::Point2<f32> {
            let m: [f32; 2] = self.into();
            mint::Point2::from(m)
        }
    }

    impl From<mint::Vector2<f32>> for Vec2 {
        fn from(m: mint::Vector2<f32>) -> Self {
            let m: [f32; 2] = m.into();
            Vec2::from(m)
        }
    }

    impl Into<mint::Vector2<f32>> for Vec2 {
        fn into(self) -> mint::Vector2<f32> {
            let m: [f32; 2] = self.into();
            mint::Vector2::from(m)
        }
    }
}
