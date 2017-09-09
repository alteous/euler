use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::SquareMatrix;
use Vec2;

/// 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat2(pub(crate) cgmath::Matrix2<f32>);

impl Mat2 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat2(cgmath::Matrix2::identity())
    }

    /// Computes the inverse of this matrix.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is not invertible.
    pub fn invert(self) -> Mat2 {
        self.try_invert().unwrap()
    }

    /// Computes the inverse of this matrix, returning `None` if non-invertible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat2!();
    /// assert_eq!(m.try_invert(), Some(mat2!()));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat2!(
    ///     0.0, 0.0,
    ///     0.0, 0.0,
    /// );
    /// assert_eq!(m.try_invert(), None);
    /// # }
    /// ```
    pub fn try_invert(self) -> Option<Mat2> {
        self.0.invert().map(Mat2)
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

impl ApproxEq for Mat2 {
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
