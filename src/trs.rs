use cgmath;
use std::fmt;

use crate::{DMat4, DQuat, DVec3, Mat4, Quat, Vec3};
use approx::ApproxEq;

/// Single-precision translation + rotation + non-uniform scale transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trs {
    /// Translation vector.
    pub t: Vec3,

    /// Rotation quaternion.
    pub r: Quat,

    /// *Non-uniform* scale factor.
    pub s: Vec3,
}

impl fmt::Display for Trs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.t, self.r, self.s))
    }
}

impl Default for Trs {
    fn default() -> Self {
        Self::identity()
    }
}

impl Trs {
    /// Full constructor.
    pub fn new(t: Vec3, r: Quat, s: Vec3) -> Self {
        Trs { t, r, s }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Trs {
            t: vec3!(),
            r: quat!(),
            s: vec3!(1.0),
        }
    }

    /// Returns the equivalent matrix representation for this transform.
    pub fn matrix(&self) -> Mat4 {
        let t =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.t.x, self.t.y, self.t.z));
        let r = cgmath::Matrix4::from(cgmath::Quaternion::new(
            self.r.s, self.r.x, self.r.y, self.r.z,
        ));
        let s = cgmath::Matrix4::from_nonuniform_scale(self.s.x, self.s.y, self.s.z);
        let m: [[f32; 4]; 4] = (t * r * s).into();
        Mat4::from(m)
    }
}

impl ApproxEq for Trs {
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
        self.t.relative_eq(&other.t, epsilon, max_relative)
            && self.r.relative_eq(&other.r, epsilon, max_relative)
            && self.s.relative_eq(&other.s, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.t.ulps_eq(&other.t, epsilon, max_ulps)
            && self.r.ulps_eq(&other.r, epsilon, max_ulps)
            && self.s.ulps_eq(&other.s, epsilon, max_ulps)
    }
}

/// Double-precision translation + rotation + non-uniform scale transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DTrs {
    /// Translation vector.
    pub t: DVec3,

    /// Rotation quaternion.
    pub r: DQuat,

    /// *Non-uniform* scale factor.
    pub s: DVec3,
}

impl fmt::Display for DTrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.t, self.r, self.s))
    }
}

impl Default for DTrs {
    fn default() -> Self {
        Self::identity()
    }
}

impl DTrs {
    /// Full constructor.
    pub fn new(t: DVec3, r: DQuat, s: DVec3) -> Self {
        DTrs { t, r, s }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        DTrs {
            t: dvec3!(),
            r: dquat!(),
            s: dvec3!(1.0),
        }
    }

    /// Returns the equivalent matrix representation for this transform.
    pub fn matrix(&self) -> DMat4 {
        let t =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.t.x, self.t.y, self.t.z));
        let r = cgmath::Matrix4::from(cgmath::Quaternion::new(
            self.r.s, self.r.x, self.r.y, self.r.z,
        ));
        let s = cgmath::Matrix4::from_nonuniform_scale(self.s.x, self.s.y, self.s.z);
        let m: [[f64; 4]; 4] = (t * r * s).into();
        DMat4::from(m)
    }
}

impl ApproxEq for DTrs {
    type Epsilon = <f64 as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        <f64 as ApproxEq>::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        <f64 as ApproxEq>::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        <f64 as ApproxEq>::default_max_ulps()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.t.relative_eq(&other.t, epsilon, max_relative)
            && self.r.relative_eq(&other.r, epsilon, max_relative)
            && self.s.relative_eq(&other.s, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.t.ulps_eq(&other.t, epsilon, max_ulps)
            && self.r.ulps_eq(&other.r, epsilon, max_ulps)
            && self.s.ulps_eq(&other.s, epsilon, max_ulps)
    }
}
