use cgmath;
use std::{mem, ops};

use cgmath::SquareMatrix;
use Vec2;

/// 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat2(cgmath::Matrix2<f32>);

impl Mat2 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat2(cgmath::Matrix2::identity())
    }

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

impl From<[[f32; 2]; 2]> for Mat2 {
    fn from(m: [[f32; 2]; 2]) -> Self {
        Mat2(m.into())
    }
}

impl Into<[[f32; 2]; 2]> for Mat2 {
    fn into(self) -> [[f32; 2]; 2] {
        self.0.into()
    }
}

impl ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Self::Output {
        Mat2(self.0 * rhs.0)
    }
}

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        let v = self.0 * cgmath::Vector2::new(rhs.x, rhs.y);
        vec2!(v.x, v.y)
    }
}

impl<'a> ops::Mul<Vec2> for &'a Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        (*self).mul(rhs)
    }
}

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Mat2;

    impl From<mint::ColumnMatrix2<f32>> for Mat2 {
        fn from(m: mint::ColumnMatrix2<f32>) -> Self {
            Mat2(m.into())
        }
    }

    impl Into<mint::ColumnMatrix2<f32>> for Mat2 {
        fn into(self) -> mint::ColumnMatrix2<f32> {
            self.0.into()
        }
    }
}
