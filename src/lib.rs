#![allow(unused_macros)]

extern crate cgmath;

use std::mem;

#[macro_use]
mod macros;

/// 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
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
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[doc(hidden)]
    pub w: f32,
}

/// Homogeneous 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
}
