use cgmath;
use mint;
use std::{mem, ops};

use approx::ApproxEq;
use cgmath::SquareMatrix;
use Vec3;

/// 3x3 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat3(pub(crate) cgmath::Matrix3<f32>);

impl Mat3 {
    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Mat3(cgmath::Matrix3::identity())
    }

    /// Computes the inverse of this matrix.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is not invertible.
    pub fn invert(self) -> Mat3 {
        self.try_invert().unwrap()
    }

    /// Computes the inverse of this matrix, returning `None` if non-invertible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat3!();
    /// assert_eq!(m.try_invert(), Some(mat3!()));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate euler;
    /// # fn main() {
    /// let m = mat3!(
    ///     0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0,
    /// );
    /// assert_eq!(m.try_invert(), None);
    /// # }
    /// ```
    pub fn try_invert(self) -> Option<Mat3> {
        self.0.invert().map(Mat3)
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

impl ApproxEq for Mat3 {
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
