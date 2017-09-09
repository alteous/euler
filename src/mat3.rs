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
        self.0.invert().map(|m| Mat3(m))
    }
}

impl AsRef<[[f32; 3]; 3]> for Mat3 {
    fn as_ref(&self) -> &[[f32; 3]; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(m: [[f32; 3]; 3]) -> Self {
        Mat3(m.into())
    }
}

impl Into<[[f32; 3]; 3]> for Mat3 {
    fn into(self) -> [[f32; 3]; 3] {
        self.0.into()
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Self::Output {
        Mat3(self.0 * rhs.0)
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let v = self.0 * cgmath::Vector3::new(rhs.x, rhs.y, rhs.z);
        vec3!(v.x, v.y, v.z)
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

    impl From<mint::ColumnMatrix3<f32>> for Mat3 {
        fn from(m: mint::ColumnMatrix3<f32>) -> Self {
            Mat3(m.into())
        }
    }

    impl Into<mint::ColumnMatrix3<f32>> for Mat3 {
        fn into(self) -> mint::ColumnMatrix3<f32> {
            self.0.into()
        }
    }
}
