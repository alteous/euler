use cgmath;
use std::{mem, ops};

use cgmath::SquareMatrix;
use {Transform, Vec4};

/// 4x4 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat4(cgmath::Matrix4<f32>);

impl Mat4 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat4(cgmath::Matrix4::identity())
    }

    /// Computes the inverse of this matrix.
    ///
    /// Returns `None` is the matrix has no inverse, i.e. has a zero determinant.
    pub fn invert(self) -> Option<Mat4> {
        self.0.invert().map(|m| Mat4(m))
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4(self.0 * rhs.0)
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let v = self.0 * cgmath::Vector4::new(rhs.x, rhs.y, rhs.z, rhs.w);
        vec4!(v.x, v.y, v.z, v.w)
    }
}

impl<'a> ops::Mul<Vec4> for &'a Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl AsRef<[[f32; 4]; 4]> for Mat4 {
    fn as_ref(&self) -> &[[f32; 4]; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<Transform> for Mat4 {
    fn from(transform: Transform) -> Self {
        transform.matrix()
    }
}

impl From<cgmath::Matrix4<f32>> for Mat4 {
    fn from(m: cgmath::Matrix4<f32>) -> Self {
        Mat4(m)
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(m: [[f32; 4]; 4]) -> Self {
        Mat4(m.into())
    }
}

impl Into<[[f32; 4]; 4]> for Mat4 {
    fn into(self) -> [[f32; 4]; 4] {
        self.0.into()
    }
}

#[cfg(feature = "mint-support")]
mod mint_support {
    use mint;
    use super::Mat4;

    #[cfg(feature = "mint-support")]
    impl From<mint::ColumnMatrix4<f32>> for Mat4 {
        fn from(m: mint::ColumnMatrix4<f32>) -> Self {
            Mat4(m.into())
        }
    }

    #[cfg(feature = "mint-support")]
    impl Into<mint::ColumnMatrix4<f32>> for Mat4 {
        fn into(self) -> mint::ColumnMatrix4<f32> {
            self.0.into()
        }
    }
}
