use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::InnerSpace;
use {Vec3, Vec4};

/// Single-precision 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,
}

impl From<Vec3> for $self {
    fn from(vec3: Vec3) -> $self {
        $self { x: vec3.x, y: vec3.y }
    }
}

impl From<Vec4> for $self {
    fn from(vec4: Vec4) -> $self {
        $self { x: vec4.x, y: vec4.y }
    }
}

/// Double-precision 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct DVec2 {
    /// X co-ordinate.
    pub x: f64,

    /// Y co-ordinate.
    pub y: f64,
}

impl From<DVec3> for DVec2 {
    fn from(dvec3: Vec3) -> Self {
        DVec2 { x: dvec3.x, y: dvec3.y }
    }
}

impl From<DVec4> for Self {
    fn from(dvec4: DVec4) -> Self {
        DVec2 { x: dvec4.x, y: dvec4.y }
    }
}

macro_rules! impl_vec {
    ($self:ident, $ty:ident) => {
        impl $self {
            /// Returns the dot product of two vectors.
            pub fn dot(self, other: $self) -> $ty {
                let left = cgmath::Vector2::new(self.x, self.y);
                let right = cgmath::Vector2::new(other.x, other.y);
                left.dot(right)
            }

            /// Returns the length of the vector.
            pub fn len(self) -> $ty {
                self.dot(self).sqrt()
            }

            /// Returns a vector in the same direction but with unit magnitude.
            pub fn normalize(self) -> $self {
                let n = cgmath::Vector2::new(self.x, self.y).normalize();
                vec2!(n.x, n.y)
            }
        }

        impl AsRef<[$ty; 2]> for $self {
            fn as_ref(&self) -> &[$ty; 2] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }

        impl From<[$ty; 2]> for $self {
            fn from(v: [$ty; 2]) -> $self {
                vec2!(v[0], v[1])
            }
        }

        impl Into<[$ty; 2]> for $self {
            fn into(self) -> [$ty; 2] {
                [self.x, self.y]
            }
        }

        impl ops::Add<$self> for $self {
            type Output = $self;
            fn add(self, rhs: $self) -> Self::Output {
                $self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl ops::Sub<$self> for $self {
            type Output = $self;
            fn sub(self, rhs: $self) -> Self::Output {
                $self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl ApproxEq for $self {
            type Epsilon = <$ty as ApproxEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                <$ty as ApproxEq>::default_epsilon()
            }

            fn default_max_relative() -> Self::Epsilon {
                <$ty as ApproxEq>::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                <$ty as ApproxEq>::default_max_ulps()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <$ty as ApproxEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
                    &&
                <$ty as ApproxEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                <$ty as ApproxEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
                    &&
                <$ty as ApproxEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            }
        }

        impl From<mint::Point2<$ty>> for $self {
            fn from(m: mint::Point2<$ty>) -> Self {
                let m: [$ty; 2] = m.into();
                $self::from(m)
            }
        }

        impl Into<mint::Point2<$ty>> for $self {
            fn into(self) -> mint::Point2<$ty> {
                let m: [$ty; 2] = self.into();
                mint::Point2::from(m)
            }
        }

        impl From<mint::Vector2<$ty>> for $self {
            fn from(m: mint::Vector2<$ty>) -> Self {
                let m: [$ty; 2] = m.into();
                $self::from(m)
            }
        }

        impl Into<mint::Vector2<$ty>> for $self {
            fn into(self) -> mint::Vector2<$ty> {
                let m: [$ty; 2] = self.into();
                mint::Vector2::from(m)
            }
        }
    };
}

impl_vec!(Vec2, f32);
impl_vec!(DVec2, f64);

