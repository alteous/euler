use cgmath;
use std::{fmt, mem, ops};

use crate::{DVec3, Vec3};
use approx::ApproxEq;
use cgmath::{InnerSpace, Rotation3};

/// Single-precision quaternion.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}; {})", self.x, self.y, self.z, self.s)
    }
}

impl Quat {
    /// Full constructor.
    pub fn new(x: f32, y: f32, z: f32, s: f32) -> Self {
        Quat { x, y, z, s }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Constructor for a rotation defined by a set of Euler angles
    ///
    /// The rotation order is Z, then X, then Y. From the point of the
    /// object, this is equivalent to a yaw in `angles.y`, a pitch in
    /// `angles.x`, and a roll in `angles.z`.
    pub fn euler(angles: Vec3) -> Self {
        let roll = quat!(0.0, 0.0, 1.0; angles.z);
        let pitch = quat!(1.0, 0.0, 0.0; angles.x);
        let yaw = quat!(0.0, 1.0, 0.0; angles.y);
        roll * pitch * yaw
    }

    /// Constructor for a rotation around `axis` by `angle` radians.
    ///
    /// `axis` need not be normalized.
    pub fn axis_angle(axis: Vec3, angle: f32) -> Self {
        let q = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::new(axis.x, axis.y, axis.z).normalize(),
            cgmath::Rad(angle),
        );
        Quat::new(q.v.x, q.v.y, q.v.z, q.s)
    }

    /// Return the application of the rotation represented by this quaternion
    /// to the vector argument.
    pub fn rotate(&self, vector: Vec3) -> Vec3 {
        use cgmath::Rotation;
        let rotation = cgmath::Quaternion::new(self.s, self.x, self.y, self.z);
        let point = cgmath::Point3::new(vector.x, vector.y, vector.z);
        let result = rotation.rotate_point(point);
        vec3!(result.x, result.y, result.z)
    }
}

/// Double-precision quaternion.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct DQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub s: f64,
}

impl fmt::Display for DQuat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}; {})", self.x, self.y, self.z, self.s)
    }
}

impl DQuat {
    /// Full constructor.
    pub fn new(x: f64, y: f64, z: f64, s: f64) -> Self {
        DQuat { x, y, z, s }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        DQuat::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Constructor for a rotation defined by a set of Euler angles
    ///
    /// The rotation order is Z, then X, then Y. From the point of the
    /// object, this is equivalent to a yaw in `angles.y`, a pitch in
    /// `angles.x`, and a roll in `angles.z`.
    pub fn euler(angles: DVec3) -> Self {
        let roll = dquat!(0.0, 0.0, 1.0; angles.z);
        let pitch = dquat!(1.0, 0.0, 0.0; angles.x);
        let yaw = dquat!(0.0, 1.0, 0.0; angles.y);
        roll * pitch * yaw
    }

    /// Constructor for a rotation around `axis` by `angle` radians.
    ///
    /// `axis` need not be normalized.
    pub fn axis_angle(axis: DVec3, angle: f64) -> Self {
        let q = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::new(axis.x, axis.y, axis.z).normalize(),
            cgmath::Rad(angle),
        );
        DQuat::new(q.v.x, q.v.y, q.v.z, q.s)
    }

    /// Return the application of the rotation represented by this quaternion
    /// to the vector argument.
    pub fn rotate(&self, vector: DVec3) -> DVec3 {
        use cgmath::Rotation;
        let rotation = cgmath::Quaternion::new(self.s, self.x, self.y, self.z);
        let point = cgmath::Point3::new(vector.x, vector.y, vector.z);
        let result = rotation.rotate_point(point);
        dvec3!(result.x, result.y, result.z)
    }
}

macro_rules! impl_quaternion {
    ($self:ty, $base:ty, $inner:ty, $array:ty) => {
        impl ops::Mul<$self> for $self {
            type Output = $self;
            fn mul(self, rhs: $self) -> $self {
                let a: &$inner = self.as_ref().into();
                let b: &$inner = rhs.as_ref().into();
                let q = a * b;
                Self::new(q.v.x, q.v.y, q.v.z, q.s)
            }
        }

        impl ops::MulAssign<$self> for $self {
            fn mul_assign(&mut self, rhs: $self) {
                *self = *self * rhs;
            }
        }

        impl Default for $self {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl AsRef<$array> for $self {
            fn as_ref(&self) -> &$array {
                unsafe { mem::transmute(self) }
            }
        }

        impl From<$array> for $self {
            fn from(q: $array) -> Self {
                Self::new(q[0], q[1], q[2], q[3])
            }
        }

        impl Into<$array> for $self {
            fn into(self) -> $array {
                [self.x, self.y, self.z, self.s]
            }
        }

        impl ApproxEq for $self {
            type Epsilon = <$inner as ApproxEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                <$inner as ApproxEq>::default_epsilon()
            }

            fn default_max_relative() -> Self::Epsilon {
                <$inner as ApproxEq>::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                <$inner as ApproxEq>::default_max_ulps()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                let a: &$inner = self.as_ref().into();
                let b: &$inner = other.as_ref().into();
                a.relative_eq(&b, epsilon, max_relative)
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                let a: &$inner = self.as_ref().into();
                let b: &$inner = other.as_ref().into();
                a.ulps_eq(b, epsilon, max_ulps)
            }
        }
    };
}

impl_quaternion!(DQuat, f64, cgmath::Quaternion<f64>, [f64; 4]);
impl_quaternion!(Quat, f32, cgmath::Quaternion<f32>, [f32; 4]);

#[cfg(feature = "mint")]
mod mint_support {
    use super::*;
    use mint;

    impl From<mint::Quaternion<f32>> for Quat {
        fn from(m: mint::Quaternion<f32>) -> Self {
            Quat::new(m.v.x, m.v.y, m.v.z, m.s)
        }
    }

    impl Into<mint::Quaternion<f32>> for Quat {
        fn into(self) -> mint::Quaternion<f32> {
            mint::Quaternion {
                v: mint::Vector3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                },
                s: self.s,
            }
        }
    }

    impl From<mint::Quaternion<f64>> for DQuat {
        fn from(m: mint::Quaternion<f64>) -> Self {
            DQuat::new(m.v.x, m.v.y, m.v.z, m.s)
        }
    }

    impl Into<mint::Quaternion<f64>> for DQuat {
        fn into(self) -> mint::Quaternion<f64> {
            mint::Quaternion {
                v: mint::Vector3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                },
                s: self.s,
            }
        }
    }
}
