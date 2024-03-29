use crate::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};
use approx::ApproxEq;
use cgmath;
use std::{fmt, mem, ops};

/// Single-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat2 {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}; {}, {})",
            self.m00, self.m01, self.m10, self.m11,
        )
    }
}

impl Mat2 {
    /// Full constructor.
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Mat2 { m00, m01, m10, m11 }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f32) -> Self {
        Mat2::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f32, di: f32, up: f32) -> Self {
        Mat2::new(di, up, lo, di)
    }
}

impl From<f32> for Mat2 {
    fn from(arg: f32) -> Self {
        Mat2::diagonal(arg)
    }
}

impl From<f64> for Mat2 {
    fn from(arg: f64) -> Self {
        Mat2::diagonal(arg as f32)
    }
}

impl From<DMat2> for Mat2 {
    fn from(arg: DMat2) -> Self {
        Mat2::new(
            arg.m00 as f32,
            arg.m01 as f32,
            arg.m10 as f32,
            arg.m11 as f32,
        )
    }
}

impl From<Mat3> for Mat2 {
    fn from(arg: Mat3) -> Self {
        Mat2::new(arg.m00, arg.m01, arg.m10, arg.m11)
    }
}

impl From<DMat3> for Mat2 {
    fn from(arg: DMat3) -> Self {
        Mat2::from(Mat3::from(arg))
    }
}

impl From<Mat4> for Mat2 {
    fn from(arg: Mat4) -> Self {
        Mat2::from(Mat3::from(arg))
    }
}

impl From<DMat4> for Mat2 {
    fn from(arg: DMat4) -> Self {
        Mat2::from(Mat3::from(arg))
    }
}

/// Double-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct DMat2 {
    pub m00: f64,
    pub m01: f64,
    pub m10: f64,
    pub m11: f64,
}

impl fmt::Display for DMat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}; {}, {})",
            self.m00, self.m01, self.m10, self.m11,
        )
    }
}

impl DMat2 {
    /// Full constructor.
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Self {
        DMat2 { m00, m01, m10, m11 }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f64) -> Self {
        DMat2::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f64, di: f64, up: f64) -> Self {
        DMat2::new(di, up, lo, di)
    }
}

impl From<f32> for DMat2 {
    fn from(arg: f32) -> Self {
        DMat2::diagonal(arg as f64)
    }
}

impl From<f64> for DMat2 {
    fn from(arg: f64) -> Self {
        DMat2::diagonal(arg)
    }
}

impl From<Mat2> for DMat2 {
    fn from(arg: Mat2) -> Self {
        DMat2::new(
            arg.m00 as f64,
            arg.m01 as f64,
            arg.m10 as f64,
            arg.m11 as f64,
        )
    }
}

impl From<Mat3> for DMat2 {
    fn from(arg: Mat3) -> Self {
        DMat2::from(DMat3::from(arg))
    }
}

impl From<DMat3> for DMat2 {
    fn from(arg: DMat3) -> Self {
        DMat2::new(arg.m00, arg.m01, arg.m10, arg.m11)
    }
}

impl From<Mat4> for DMat2 {
    fn from(arg: Mat4) -> Self {
        DMat2::from(DMat3::from(arg))
    }
}

impl From<DMat4> for DMat2 {
    fn from(arg: DMat4) -> Self {
        DMat2::from(DMat3::from(arg))
    }
}

/// Single-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}; {}, {}, {}; {}, {}, {})",
            self.m00,
            self.m01,
            self.m02,
            self.m10,
            self.m11,
            self.m12,
            self.m20,
            self.m21,
            self.m22,
        )
    }
}

impl Mat3 {
    /// Full constructor.
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Mat3 {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f32) -> Self {
        Mat3::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f32, di: f32, up: f32) -> Self {
        Mat3::new(di, up, 0., lo, di, up, 0., lo, di)
    }
}

impl From<f32> for Mat3 {
    fn from(arg: f32) -> Self {
        Self {
            m00: arg,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: arg,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: arg,
        }
    }
}

impl From<f64> for Mat3 {
    fn from(arg: f64) -> Self {
        Mat3::from(arg as f32)
    }
}

impl From<Mat2> for Mat3 {
    fn from(arg: Mat2) -> Self {
        Mat3::new(arg.m00, arg.m01, 0.0, arg.m10, arg.m11, 0.0, 0.0, 0.0, 1.0)
    }
}

impl From<DMat2> for Mat3 {
    fn from(arg: DMat2) -> Self {
        Mat3::from(Mat2::from(arg))
    }
}

impl From<DMat3> for Mat3 {
    fn from(arg: DMat3) -> Self {
        Mat3::new(
            arg.m00 as f32,
            arg.m01 as f32,
            arg.m02 as f32,
            arg.m10 as f32,
            arg.m11 as f32,
            arg.m12 as f32,
            arg.m20 as f32,
            arg.m21 as f32,
            arg.m22 as f32,
        )
    }
}

impl From<Mat4> for Mat3 {
    fn from(arg: Mat4) -> Self {
        Mat3::new(
            arg.m00, arg.m01, arg.m02, arg.m10, arg.m11, arg.m12, arg.m20, arg.m21, arg.m22,
        )
    }
}

impl From<DMat4> for Mat3 {
    fn from(arg: DMat4) -> Self {
        Mat3::from(Mat4::from(arg))
    }
}

/// Double-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct DMat3 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
}

impl fmt::Display for DMat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}; {}, {}, {}; {}, {}, {})",
            self.m00,
            self.m01,
            self.m02,
            self.m10,
            self.m11,
            self.m12,
            self.m20,
            self.m21,
            self.m22,
        )
    }
}

impl DMat3 {
    /// Full constructor.
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Self {
        DMat3 {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f64) -> Self {
        DMat3::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f64, di: f64, up: f64) -> Self {
        DMat3::new(di, up, 0., lo, di, up, 0., lo, di)
    }
}

impl From<f32> for DMat3 {
    fn from(arg: f32) -> Self {
        DMat3::diagonal(arg as f64)
    }
}

impl From<f64> for DMat3 {
    fn from(arg: f64) -> Self {
        DMat3::diagonal(arg)
    }
}

impl From<Mat2> for DMat3 {
    fn from(arg: Mat2) -> Self {
        DMat3::from(DMat2::from(arg))
    }
}

impl From<DMat2> for DMat3 {
    fn from(arg: DMat2) -> Self {
        DMat3::new(arg.m00, arg.m01, 0.0, arg.m10, arg.m11, 0.0, 0.0, 0.0, 1.0)
    }
}

impl From<Mat3> for DMat3 {
    fn from(arg: Mat3) -> Self {
        DMat3::new(
            arg.m00 as f64,
            arg.m01 as f64,
            arg.m02 as f64,
            arg.m10 as f64,
            arg.m11 as f64,
            arg.m12 as f64,
            arg.m20 as f64,
            arg.m21 as f64,
            arg.m22 as f64,
        )
    }
}

impl From<Mat4> for DMat3 {
    fn from(arg: Mat4) -> Self {
        DMat3::from(DMat4::from(arg))
    }
}

impl From<DMat4> for DMat3 {
    fn from(arg: DMat4) -> Self {
        DMat3::new(
            arg.m00, arg.m01, arg.m02, arg.m10, arg.m11, arg.m12, arg.m20, arg.m21, arg.m22,
        )
    }
}

/// Single-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Mat4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {})",
            self.m00,
            self.m01,
            self.m02,
            self.m03,
            self.m10,
            self.m11,
            self.m12,
            self.m13,
            self.m20,
            self.m21,
            self.m22,
            self.m23,
            self.m30,
            self.m31,
            self.m32,
            self.m33,
        )
    }
}

impl Mat4 {
    /// Full constructor.
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Self {
        Mat4 {
            m00,
            m01,
            m02,
            m03,
            m10,
            m11,
            m12,
            m13,
            m20,
            m21,
            m22,
            m23,
            m30,
            m31,
            m32,
            m33,
        }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f32) -> Self {
        Mat4::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f32, di: f32, up: f32) -> Self {
        Mat4::new(
            di, up, 0., 0., lo, di, up, 0., 0., lo, di, up, 0., 0., lo, di,
        )
    }
}

impl From<f32> for Mat4 {
    fn from(arg: f32) -> Self {
        Mat4::diagonal(arg)
    }
}

impl From<f64> for Mat4 {
    fn from(arg: f64) -> Self {
        Mat4::diagonal(arg as f32)
    }
}

impl From<Mat2> for Mat4 {
    fn from(arg: Mat2) -> Self {
        Mat4::from(Mat3::from(arg))
    }
}

impl From<DMat2> for Mat4 {
    fn from(arg: DMat2) -> Self {
        Mat4::from(Mat2::from(arg))
    }
}

impl From<Mat3> for Mat4 {
    fn from(arg: Mat3) -> Self {
        Mat4::new(
            arg.m00, arg.m01, arg.m02, 0.0, arg.m10, arg.m11, arg.m12, 0.0, arg.m20, arg.m21,
            arg.m22, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl From<DMat3> for Mat4 {
    fn from(arg: DMat3) -> Self {
        Mat4::from(Mat3::from(arg))
    }
}

impl From<DMat4> for Mat4 {
    fn from(arg: DMat4) -> Self {
        Mat4::new(
            arg.m00 as f32,
            arg.m01 as f32,
            arg.m02 as f32,
            arg.m03 as f32,
            arg.m10 as f32,
            arg.m11 as f32,
            arg.m12 as f32,
            arg.m13 as f32,
            arg.m20 as f32,
            arg.m21 as f32,
            arg.m22 as f32,
            arg.m23 as f32,
            arg.m30 as f32,
            arg.m31 as f32,
            arg.m32 as f32,
            arg.m33 as f32,
        )
    }
}

/// Double-precision 2x2 column major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct DMat4 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m03: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m30: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}

impl fmt::Display for DMat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {})",
            self.m00,
            self.m01,
            self.m02,
            self.m03,
            self.m10,
            self.m11,
            self.m12,
            self.m13,
            self.m20,
            self.m21,
            self.m22,
            self.m23,
            self.m30,
            self.m31,
            self.m32,
            self.m33,
        )
    }
}

impl DMat4 {
    /// Full constructor.
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Self {
        DMat4 {
            m00,
            m01,
            m02,
            m03,
            m10,
            m11,
            m12,
            m13,
            m20,
            m21,
            m22,
            m23,
            m30,
            m31,
            m32,
            m33,
        }
    }

    /// Identity constructor.
    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    /// Diagonal constructor.
    pub fn diagonal(di: f64) -> Self {
        DMat4::tridiagonal(0.0, di, 0.0)
    }

    /// Tri-diagonal constructor.
    pub fn tridiagonal(lo: f64, di: f64, up: f64) -> Self {
        DMat4::new(
            di, up, 0., 0., lo, di, up, 0., 0., lo, di, up, 0., 0., lo, di,
        )
    }
}

impl From<f32> for DMat4 {
    fn from(arg: f32) -> Self {
        DMat4::diagonal(arg as f64)
    }
}

impl From<f64> for DMat4 {
    fn from(arg: f64) -> Self {
        DMat4::diagonal(arg)
    }
}

impl From<Mat2> for DMat4 {
    fn from(arg: Mat2) -> Self {
        DMat4::from(DMat3::from(arg))
    }
}

impl From<DMat2> for DMat4 {
    fn from(arg: DMat2) -> Self {
        DMat4::from(DMat3::from(arg))
    }
}

impl From<Mat3> for DMat4 {
    fn from(arg: Mat3) -> Self {
        DMat4::from(DMat3::from(arg))
    }
}

impl From<DMat3> for DMat4 {
    fn from(arg: DMat3) -> Self {
        DMat4::new(
            arg.m00, arg.m01, arg.m02, 0.0, arg.m10, arg.m11, arg.m12, 0.0, arg.m20, arg.m21,
            arg.m22, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl From<Mat4> for DMat4 {
    fn from(arg: Mat4) -> Self {
        DMat4::new(
            arg.m00 as f64,
            arg.m01 as f64,
            arg.m02 as f64,
            arg.m03 as f64,
            arg.m10 as f64,
            arg.m11 as f64,
            arg.m12 as f64,
            arg.m13 as f64,
            arg.m20 as f64,
            arg.m21 as f64,
            arg.m22 as f64,
            arg.m23 as f64,
            arg.m30 as f64,
            arg.m31 as f64,
            arg.m32 as f64,
            arg.m33 as f64,
        )
    }
}

macro_rules! impl_matrix {
    ($self:ident, $minner:ty, $marray:ty, $vec:ty, $vinner:ty, $varray:ty, $base:ty) => {
        impl $self {
            /// Computes the matrix determinant.
            pub fn determinant(self) -> $base {
                use cgmath::SquareMatrix;
                let a: &$minner = self.as_ref().into();
                a.determinant()
            }

            /// Computes the matrix trace.
            pub fn trace(self) -> $base {
                use cgmath::SquareMatrix;
                let a: &$minner = self.as_ref().into();
                a.trace()
            }

            /// Computes the matrix inverse.
            ///
            /// ## Panics
            ///
            /// Panics if the matrix has no inverse (i.e. has zero determinant).
            pub fn inverse(self) -> $self {
                self.try_invert().unwrap()
            }

            /// Returns the matrix transpose.
            pub fn transpose(self) -> $self {
                use cgmath::SquareMatrix;
                let a: &$minner = self.as_ref().into();
                let mut b = *a;
                b.transpose_self();
                let m: $marray = b.into();
                m.into()
            }

            /// Attempts to compute the matrix inverse, returning `None` if the matrix is
            /// non-invertible (i.e. has zero determinant).
            pub fn try_invert(self) -> Option<$self> {
                use cgmath::SquareMatrix;
                let a: &$minner = self.as_ref().into();
                a.invert().map(|inv| {
                    let b: $marray = inv.into();
                    b.into()
                })
            }
        }

        impl ops::Add<$self> for $self {
            type Output = $self;
            fn add(self, rhs: $self) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let b: &$minner = rhs.as_ref().into();
                let m: $marray = (a + b).into();
                m.into()
            }
        }

        impl ops::Sub<$self> for $self {
            type Output = $self;
            fn sub(self, rhs: $self) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let b: &$minner = rhs.as_ref().into();
                let m: $marray = (a - b).into();
                m.into()
            }
        }

        impl ops::Mul<$base> for $self {
            type Output = $self;
            fn mul(self, rhs: $base) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let v: $marray = (a * rhs).into();
                v.into()
            }
        }

        impl ops::Mul<$vec> for $self {
            type Output = $vec;
            fn mul(self, rhs: $vec) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let b: &$vinner = rhs.as_ref().into();
                let v: $varray = (a * b).into();
                v.into()
            }
        }

        impl<'a> ops::Mul<$vec> for &'a $self {
            type Output = $vec;
            fn mul(self, rhs: $vec) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let b: &$vinner = rhs.as_ref().into();
                let v: $varray = (a * b).into();
                v.into()
            }
        }

        impl ops::Mul<$self> for $base {
            type Output = $self;
            fn mul(self, rhs: $self) -> Self::Output {
                ops::Mul::mul(rhs, self)
            }
        }

        impl ops::Mul<$self> for $self {
            type Output = $self;
            fn mul(self, rhs: $self) -> Self::Output {
                let a: &$minner = self.as_ref().into();
                let b: &$minner = rhs.as_ref().into();
                let v: $marray = (a * b).into();
                v.into()
            }
        }

        impl Default for $self {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl AsRef<$marray> for $self {
            fn as_ref(&self) -> &$marray {
                unsafe { mem::transmute(self) }
            }
        }

        impl From<$marray> for $self {
            fn from(array: $marray) -> Self {
                unsafe { mem::transmute(array) }
            }
        }

        impl Into<$marray> for $self {
            fn into(self) -> $marray {
                unsafe { mem::transmute(self) }
            }
        }

        impl ApproxEq for $self {
            type Epsilon = <$minner as ApproxEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                <$minner as ApproxEq>::default_epsilon()
            }

            fn default_max_relative() -> Self::Epsilon {
                <$minner as ApproxEq>::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                <$minner as ApproxEq>::default_max_ulps()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                let a: &$minner = self.as_ref().into();
                let b: &$minner = other.as_ref().into();
                a.relative_eq(&b, epsilon, max_relative)
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                let a: &$minner = self.as_ref().into();
                let b: &$minner = other.as_ref().into();
                a.ulps_eq(&b, epsilon, max_ulps)
            }
        }
    };
}

impl_matrix!(
    Mat2,
    cgmath::Matrix2<f32>,
    [[f32; 2]; 2],
    Vec2,
    cgmath::Vector2<f32>,
    [f32; 2],
    f32
);
impl_matrix!(
    Mat3,
    cgmath::Matrix3<f32>,
    [[f32; 3]; 3],
    Vec3,
    cgmath::Vector3<f32>,
    [f32; 3],
    f32
);
impl_matrix!(
    Mat4,
    cgmath::Matrix4<f32>,
    [[f32; 4]; 4],
    Vec4,
    cgmath::Vector4<f32>,
    [f32; 4],
    f32
);

impl_matrix!(
    DMat2,
    cgmath::Matrix2<f64>,
    [[f64; 2]; 2],
    DVec2,
    cgmath::Vector2<f64>,
    [f64; 2],
    f64
);
impl_matrix!(
    DMat3,
    cgmath::Matrix3<f64>,
    [[f64; 3]; 3],
    DVec3,
    cgmath::Vector3<f64>,
    [f64; 3],
    f64
);
impl_matrix!(
    DMat4,
    cgmath::Matrix4<f64>,
    [[f64; 4]; 4],
    DVec4,
    cgmath::Vector4<f64>,
    [f64; 4],
    f64
);

#[cfg(feature = "mint")]
mod mint_support {
    use super::*;
    use mint;

    macro_rules! impl_mint_conversion {
        ($self:ty, $mint:ty, $via:ty) => {
            impl From<$mint> for $self {
                fn from(m: $mint) -> Self {
                    let v: $via = m.into();
                    v.into()
                }
            }

            impl Into<$mint> for $self {
                fn into(self) -> $mint {
                    let v: $via = self.into();
                    v.into()
                }
            }
        };
    }

    impl_mint_conversion!(Mat2, mint::ColumnMatrix2<f32>, [[f32; 2]; 2]);
    impl_mint_conversion!(Mat3, mint::ColumnMatrix3<f32>, [[f32; 3]; 3]);
    impl_mint_conversion!(Mat4, mint::ColumnMatrix4<f32>, [[f32; 4]; 4]);

    impl_mint_conversion!(DMat2, mint::ColumnMatrix2<f64>, [[f64; 2]; 2]);
    impl_mint_conversion!(DMat3, mint::ColumnMatrix3<f64>, [[f64; 3]; 3]);
    impl_mint_conversion!(DMat4, mint::ColumnMatrix4<f64>, [[f64; 4]; 4]);
}
