use cgmath;
use std::{mem, ops};

use cgmath::SquareMatrix;
use Vec4;

/// 4x4 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat4(cgmath::Matrix4<f32>);

impl Mat4 {
    /// Computes the inverse of this matrix.
    ///
    /// Returns `None` is the matrix has no inverse, i.e. has a zero determinant.
    pub fn invert(self) -> Option<Mat4> {
        self.0.invert().map(|m| m.into())
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let matrix: cgmath::Matrix4<f32> = unsafe { mem::transmute(self) };
        let vector: cgmath::Vector4<f32> = unsafe { mem::transmute(rhs) };
        let output: [f32; 4] = (matrix * vector).into();
        unsafe { mem::transmute(output) }
    }
}

impl AsRef<[[f32; 4]; 4]> for Mat4 {
    fn as_ref(&self) -> &[[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<cgmath::Matrix4<f32>> for Mat4 {
    fn from(m: cgmath::Matrix4<f32>) -> Self {
        Mat4(m)
    }
}

impl Into<[[f32; 4]; 4]> for Mat4 {
    fn into(self) -> [[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}
