#[macro_use]
extern crate approx;
extern crate cgmath;

use std::{mem, ops};

use approx::ApproxEq;

#[macro_use]
mod macros;

/// Translation + Rotation + Non-uniform Scale transform in 3D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3 {
    /// Translation vector.
    pub translation: Vec3,

    /// Rotation quaternion.
    pub rotation: Quaternion,

    /// *Non-uniform* scaling.
    pub scale: Vec3,
}

impl Transform3 {
    /// Returns the identity transform.
    pub fn identity() -> Self {
        Transform3 {
            translation: vec3!(),
            rotation: Quaternion::identity(),
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

/// Quaternion in the order `[x, y, z, w]`, where `w` is the scalar.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    /// Vector part.
    pub vector: Vec3,

    /// Scalar part.
    pub scalar: f32,
}

impl Quaternion {
    /// Returns the identity quaternion.
    pub fn identity() -> Self {
        Quaternion {
            scalar: 1.0,
            vector: vec3!(),
        }
    }
}

/// 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    /// X co-ordinate.
    pub x: f32,

    /// Y co-ordinate.
    pub y: f32,
}

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

    /// Homogeneous W co-ordinate - always `1.0` for 3D vectors.
    #[doc(hidden)]
    pub w: f32,
}

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

/// 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat2 {
    pub m00: f32,
    pub m01: f32,

    pub m10: f32,
    pub m11: f32,
}

/// 3x3 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,

    pub m10: f32,
    pub m11: f32,
    pub m12: f32,

    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

/// 4x4 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,

    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,

    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,

    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
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

impl ops::Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
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

impl From<Vec4> for Vec3 {
    fn from(vec4: Vec4) -> Vec3 {
        Vec3 { x: vec4.x, y: vec4.y, z: vec4.z, w: 0.0 }
    }
}

impl AsRef<[[f32; 2]; 2]> for Mat2 {
    fn as_ref(&self) -> &[[f32; 2]; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[[f32; 3]; 3]> for Mat3 {
    fn as_ref(&self) -> &[[f32; 3]; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[[f32; 4]; 4]> for Mat4 {
    fn as_ref(&self) -> &[[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[f32; 4]> for Quaternion {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    fn as_ref(&self) -> &[f32; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mat2_macro_empty() {
        let empty = mat2!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0],
                [0.0, 1.0],
            ]
        );
    }

    #[test]
    fn mat2_macro_single() {
        let single = mat2!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0],
                [0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat2_macro_full() {
        let full = mat2!(
            1.2, 3.4,
            5.6, 7.8,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [1.2, 3.4],
                [5.6, 7.8],
            ]
        );
    }

    #[test]
    fn mat3_macro_empty() {
        let empty = mat3!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        );
    }
    #[test]
    fn mat3_macro_single() {
        let single = mat3!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
                [0.0, 0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat3_macro_full() {
        let full = mat3!(
            0.1, 0.2, 0.3,
            0.4, 0.5, 0.6,
            0.7, 0.8, 0.9,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [0.1, 0.2, 0.3],
                [0.4, 0.5, 0.6],
                [0.7, 0.8, 0.9],
            ]
        );
    }

    #[test]
    fn mat4_macro_empty() {
        let empty = mat4!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn mat4_macro_single() {
        let single = mat4!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat4_macro_full() {
        let full = mat4!(
            0.1, 0.2, 0.3, 0.4,
            0.5, 0.6, 0.7, 0.8,
            0.9, 1.0, 1.1, 1.2,
            1.3, 1.4, 1.5, 1.6,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [0.1, 0.2, 0.3, 0.4],
                [0.5, 0.6, 0.7, 0.8],
                [0.9, 1.0, 1.1, 1.2],
                [1.3, 1.4, 1.5, 1.6],
            ]
        );
    }

    #[test]
    fn vec2_relative_eq() {
        assert_relative_eq!(vec2!(1.0, 2.0), vec2!(1.0, 2.0));
        assert_relative_ne!(vec2!(1.0, 2.0), vec2!(1.0, 2.1));
    }

    #[test]
    fn vec2_plus_scalar() {
        assert_relative_eq!(vec2!(2.2, 4.4), vec2!(1.2, 3.4) + 1.0);
    }

    #[test]
    fn vec2_minus_scalar() {
        assert_relative_eq!(vec2!(0.2, 2.4), vec2!(2.2, 4.4) - 2.0);
    }

    #[test]
    fn vec2_plus_vec2() {
        assert_relative_eq!(vec2!(1.6, 4.0), vec2!(1.2, 3.4) + vec2!(0.4, 0.6));
    }

    #[test]
    fn vec2_minus_vec2() {
        assert_relative_eq!(vec2!(), vec2!(2.2, 4.4) - vec2!(2.2, 4.4));
    }

    #[test]
    fn vec3_relative_eq() {
        assert_relative_eq!(vec3!(1.0, 2.0, 3.0), vec3!(1.0, 2.0, 3.0));
        assert_relative_ne!(vec3!(1.0, 2.0, 3.0), vec3!(1.0, 2.0, 3.1));
    }

    #[test]
    fn vec3_plus_scalar() {
        assert_relative_eq!(vec3!(2.2, 4.4, 6.6), vec3!(1.2, 3.4, 5.6) + 1.0);
    }

    #[test]
    fn vec3_minus_scalar() {
        assert_relative_eq!(vec3!(0.2, 2.4, 4.6), vec3!(1.2, 3.4, 5.6) - 1.0);
    }

    #[test]
    fn vec3_plus_vec3() {
        assert_relative_eq!(
            vec3!(2.2, 5.4, 8.6),
            vec3!(1.2, 3.4, 5.6) + vec3!(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn vec3_minus_vec3() {
        assert_relative_eq!(
            vec3!(0.2, 1.4, 2.6),
            vec3!(1.2, 3.4, 5.6) - vec3!(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn vec2_macro_empty() {
        let zero = vec2!();
        assert_eq!(zero.as_ref(), &[0.0, 0.0]);
    }    

    #[test]
    fn vec2_macro_ones() {
        let ones = vec2!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0]);
    }

    #[test]
    fn vec2_macro_full() {
        let full = vec2!(1.2, 3.4);
        assert_eq!(full.as_ref(), &[1.2, 3.4]);
    }

    #[test]
    fn vec3_macro_zeros() {
        let zeros = vec3!();
        assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0]);
        assert_eq!(zeros.w, 0.0);
    }

    #[test]
    fn vec3_macro_ones() {
        let ones = vec3!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0]);
        assert_eq!(ones.w, 0.0);
    }

    #[test]
    fn vec3_macro_full() {
        let full = vec3!(1.2, 3.4, 5.6);
        assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6]);
        
    }

    #[test]
    fn vec3_macro_from_vec2() {
        let vec2 = vec2!(1.2, 3.4);
        let vec3 = vec3!(vec2, 5.6);
        assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6]);
        assert_eq!(vec3.w, 0.0);
    }

    #[test]
    fn vec4_macro_zeros() {
        let zeros = vec4!();
        assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn vec4_macro_ones() {
        let ones = vec4!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn vec4_macro_full() {
        let full = vec4!(1.2, 3.4, 5.6, 7.8);
        assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_macro_from_vec2() {
        let vec2 = vec2!(1.2, 3.4);
        let vec4 = vec4!(vec2, 5.6, 7.8);
        assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_macro_from_vec3() {
        let vec3 = vec3!(1.2, 3.4, 5.6);
        let vec4 = vec4!(vec3, 7.8);
        assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_relative_eq() {
        assert_relative_eq!(vec4!(1.0, 2.0, 3.0, 1.0), vec4!(1.0, 2.0, 3.0, 1.0));
        assert_relative_ne!(vec4!(1.0, 2.0, 3.0, 1.0), vec4!(1.0, 2.0, 3.0, 0.0));
    }
}
