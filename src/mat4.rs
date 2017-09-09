use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::SquareMatrix;
use {Transform, Vec4};

/// 4x4 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat4(pub(crate) cgmath::Matrix4<f32>);

impl Mat4 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat4(cgmath::Matrix4::identity())
    }

    /// Computes the inverse of this matrix.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is not invertible.
    pub fn invert(self) -> Mat4 {
        self.try_invert().unwrap()
    }

    /// Computes the inverse of this matrix, returning `None` if non-invertible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat4!();
    /// assert_eq!(m.try_invert(), Some(mat4!()));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat4!(
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0,
    /// );
    /// assert_eq!(m.try_invert(), None);
    /// # }
    /// ```
    pub fn try_invert(self) -> Option<Mat4> {
        self.0.invert().map(Mat4)
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

impl ApproxEq for Mat4 {
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
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}

impl From<mint::ColumnMatrix4<f32>> for Mat4 {
    fn from(m: mint::ColumnMatrix4<f32>) -> Self {
        Mat4(m.into())
    }
}

impl Into<mint::ColumnMatrix4<f32>> for Mat4 {
    fn into(self) -> mint::ColumnMatrix4<f32> {
        self.0.into()
    }
}
