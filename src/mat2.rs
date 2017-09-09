use cgmath;
use std::mem;

use cgmath::SquareMatrix;

/// 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat2(cgmath::Matrix2<f32>);

impl Mat2 {
    /// Computes the inverse of this matrix.
    ///
    /// Returns `None` is the matrix has no inverse, i.e. has a zero determinant.
    pub fn invert(self) -> Option<Mat2> {
        self.0.invert().map(|m| m.into())
    }
}

impl AsRef<[[f32; 2]; 2]> for Mat2 {
    fn as_ref(&self) -> &[[f32; 2]; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<cgmath::Matrix2<f32>> for Mat2 {
    fn from(m: cgmath::Matrix2<f32>) -> Self {
        Mat2(m)
    }
}

impl Into<[[f32; 2]; 2]> for Mat2 {
    fn into(self) -> [[f32; 2]; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}
