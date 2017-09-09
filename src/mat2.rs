use cgmath;
use std::mem;

use cgmath::SquareMatrix;

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
        unsafe {
            mem::transmute(self)
        }
    }
}

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Mat2;

    #[cfg(feature = "mint-support")]
    impl From<mint::ColumnMatrix2<f32>> for Mat2 {
        fn from(m: mint::ColumnMatrix2<f32>) -> Self {
            let m: [[f32; 2]; 2] = m.into();
            Mat2::from(m)
        }
    }

    #[cfg(feature = "mint-support")]
    impl Into<mint::ColumnMatrix2<f32>> for Mat2 {
        fn into(self) -> mint::ColumnMatrix2<f32> {
            let m: [[f32; 2]; 2] = self.into();
            mint::ColumnMatrix2::from(m)
        }
    }
}
