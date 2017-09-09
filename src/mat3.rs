use cgmath;
use cgmath::SquareMatrix;
use std::mem;

/// 3x3 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat3(cgmath::Matrix3<f32>);

impl Mat3 {
    /// Computes the inverse of this matrix.
    ///
    /// Returns `None` is the matrix has no inverse, i.e. has a zero determinant.
    pub fn invert(self) -> Option<Mat3> {
        self.0.invert().map(|m| m.into())
    }
}

impl AsRef<[[f32; 3]; 3]> for Mat3 {
    fn as_ref(&self) -> &[[f32; 3]; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<cgmath::Matrix3<f32>> for Mat3 {
    fn from(m: cgmath::Matrix3<f32>) -> Self {
        Mat3(m)
    }
}

impl Into<[[f32; 3]; 3]> for Mat3 {
    fn into(self) -> [[f32; 3]; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}
