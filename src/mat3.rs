use cgmath;
use std::{mem, ops};

use cgmath::SquareMatrix;
use Vec3;

/// 3x3 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat3(cgmath::Matrix3<f32>);

impl Mat3 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat3(cgmath::Matrix3::identity())
    }

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

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(m: [[f32; 3]; 3]) -> Self {
        Mat3(m.into())
    }
}

impl Into<[[f32; 3]; 3]> for Mat3 {
    fn into(self) -> [[f32; 3]; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Self::Output {
        let left: cgmath::Matrix3<f32> = unsafe { mem::transmute(self) };
        let right: cgmath::Matrix3<f32> = unsafe { mem::transmute(rhs) };
        let output: [[f32; 3]; 3] = (left * right).into();
        output.into()
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let matrix: cgmath::Matrix3<f32> = unsafe { mem::transmute(self) };
        let vector: cgmath::Vector3<f32> = unsafe { mem::transmute(rhs) };
        let output: [f32; 3] = (matrix * vector).into();
        unsafe { mem::transmute(output) }
    }
}

impl<'a> ops::Mul<Vec3> for &'a Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (*self).mul(rhs)
    }
}

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Mat3;

    #[cfg(feature = "mint-support")]
    impl From<mint::ColumnMatrix3<f32>> for Mat3 {
        fn from(m: mint::ColumnMatrix3<f32>) -> Self {
            let m: [[f32; 3]; 3] = m.into();
            Mat3::from(m)
        }
    }

    #[cfg(feature = "mint-support")]
    impl Into<mint::ColumnMatrix3<f32>> for Mat3 {
        fn into(self) -> mint::ColumnMatrix3<f32> {
            let m: [[f32; 3]; 3] = self.into();
            mint::ColumnMatrix3::from(m)
        }
    }
}
