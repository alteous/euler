/// Single-precision 2x2 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = mat2!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///         [1.0, 0.0],
///         [0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = mat2!(
///     1.2, 3.4,
///     5.6, 7.8,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [1.2, 3.4],
///         [5.6, 7.8],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! mat2 {
    () => {
        $crate::Mat2::identity()
    };

    ($expr:expr) => {
        $crate::Mat2::from($expr)
    };

    (
        $m00:expr, $m01:expr,
        $m10:expr, $m11:expr,
    ) => {
        $crate::Mat2::new($m00 as f32, $m01 as f32, $m10 as f32, $m11 as f32)
    };
}

/// Double-precision 2x2 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = dmat2!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///         [1.0, 0.0],
///         [0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dmat2!(
///     1.2, 3.4,
///     5.6, 7.8,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [1.2, 3.4],
///         [5.6, 7.8],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dmat2 {
    () => {
        $crate::DMat2::identity()
    };

    ($expr:expr) => {
        $crate::DMat2::from($expr)
    };

    (
        $m00:expr, $m01:expr,
        $m10:expr, $m11:expr,
    ) => {
        $crate::DMat2::new($m00 as f64, $m01 as f64, $m10 as f64, $m11 as f64)
    };
}

/// Single-precision 3x3 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = mat3!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///         [1.0, 0.0, 0.0],
///         [0.0, 1.0, 0.0],
///         [0.0, 0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = mat3!(
///     0.1, 0.2, 0.3,
///     0.4, 0.5, 0.6,
///     0.7, 0.8, 0.9,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [0.1, 0.2, 0.3],
///         [0.4, 0.5, 0.6],
///         [0.7, 0.8, 0.9],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! mat3 {
    () => {
        $crate::Mat3::identity()
    };

    ($expr:expr) => {
        $crate::Mat3::from($expr)
    };

    (
        $m00:expr, $m01:expr, $m02:expr,
        $m10:expr, $m11:expr, $m12:expr,
        $m20:expr, $m21:expr, $m22:expr,
    ) => {
        $crate::Mat3::new(
            $m00 as f32,
            $m01 as f32,
            $m02 as f32,
            $m10 as f32,
            $m11 as f32,
            $m12 as f32,
            $m20 as f32,
            $m21 as f32,
            $m22 as f32,
        )
    };
}

/// Double-precision 3x3 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = dmat3!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///         [1.0, 0.0, 0.0],
///         [0.0, 1.0, 0.0],
///         [0.0, 0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dmat3!(
///     0.1, 0.2, 0.3,
///     0.4, 0.5, 0.6,
///     0.7, 0.8, 0.9,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [0.1, 0.2, 0.3],
///         [0.4, 0.5, 0.6],
///         [0.7, 0.8, 0.9],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dmat3 {
    () => {
        $crate::DMat3::identity()
    };

    ($expr:expr) => {
        $crate::DMat3::from($expr)
    };

    (
        $m00:expr, $m01:expr, $m02:expr,
        $m10:expr, $m11:expr, $m12:expr,
        $m20:expr, $m21:expr, $m22:expr,
    ) => {
        $crate::DMat3::new(
            $m00 as f64,
            $m01 as f64,
            $m02 as f64,
            $m10 as f64,
            $m11 as f64,
            $m12 as f64,
            $m20 as f64,
            $m21 as f64,
            $m22 as f64,
        )
    };
}

/// Single-precision 4x4 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = mat4!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///          [1.0, 0.0, 0.0, 0.0],
///          [0.0, 1.0, 0.0, 0.0],
///          [0.0, 0.0, 1.0, 0.0],
///          [0.0, 0.0, 0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = mat4!(
///     0.1, 0.2, 0.3, 0.4,
///     0.5, 0.6, 0.7, 0.8,
///     0.9, 1.0, 1.1, 1.2,
///     1.3, 1.4, 1.5, 1.6,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [0.1, 0.2, 0.3, 0.4],
///         [0.5, 0.6, 0.7, 0.8],
///         [0.9, 1.0, 1.1, 1.2],
///         [1.3, 1.4, 1.5, 1.6],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! mat4 {
    () => {
        $crate::Mat4::identity()
    };

    ($expr:expr) => {
        $crate::Mat4::from($expr)
    };

    (
        $m00:expr, $m01:expr, $m02:expr, $m03:expr,
        $m10:expr, $m11:expr, $m12:expr, $m13:expr,
        $m20:expr, $m21:expr, $m22:expr, $m23:expr,
        $m30:expr, $m31:expr, $m32:expr, $m33:expr,
    ) => {
        $crate::Mat4::new(
            $m00 as f32,
            $m01 as f32,
            $m02 as f32,
            $m03 as f32,
            $m10 as f32,
            $m11 as f32,
            $m12 as f32,
            $m13 as f32,
            $m20 as f32,
            $m21 as f32,
            $m22 as f32,
            $m23 as f32,
            $m30 as f32,
            $m31 as f32,
            $m32 as f32,
            $m33 as f32,
        )
    };
}

/// Double-precision 4x4 matrix macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let empty = dmat4!();
/// assert_eq!(
///     empty.as_ref(),
///     &[
///          [1.0, 0.0, 0.0, 0.0],
///          [0.0, 1.0, 0.0, 0.0],
///          [0.0, 0.0, 1.0, 0.0],
///          [0.0, 0.0, 0.0, 1.0],
///     ]
/// );
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dmat4!(
///     0.1, 0.2, 0.3, 0.4,
///     0.5, 0.6, 0.7, 0.8,
///     0.9, 1.0, 1.1, 1.2,
///     1.3, 1.4, 1.5, 1.6,
/// );
/// assert_eq!(
///     full.as_ref(),
///     &[
///         [0.1, 0.2, 0.3, 0.4],
///         [0.5, 0.6, 0.7, 0.8],
///         [0.9, 1.0, 1.1, 1.2],
///         [1.3, 1.4, 1.5, 1.6],
///     ]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dmat4 {
    () => {
        $crate::DMat4::identity()
    };

    ($expr:expr) => {
        $crate::DMat4::from($expr)
    };

    (
        $m00:expr, $m01:expr, $m02:expr, $m03:expr,
        $m10:expr, $m11:expr, $m12:expr, $m13:expr,
        $m20:expr, $m21:expr, $m22:expr, $m23:expr,
        $m30:expr, $m31:expr, $m32:expr, $m33:expr,
    ) => {
        $crate::DMat4::new(
            $m00 as f64,
            $m01 as f64,
            $m02 as f64,
            $m03 as f64,
            $m10 as f64,
            $m11 as f64,
            $m12 as f64,
            $m13 as f64,
            $m20 as f64,
            $m21 as f64,
            $m22 as f64,
            $m23 as f64,
            $m30 as f64,
            $m31 as f64,
            $m32 as f64,
            $m33 as f64,
        )
    };
}

/// Single-precision quaternion macro constructor.
///
/// Identity.
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let q = quat!();
/// assert_eq!(q.as_ref(), &[0.0, 0.0, 0.0, 1.0]);
/// # }
/// ```
///
/// Rotation around explicit axis values.
///
/// ```rust
/// # #[macro_use] extern crate approx;
/// # #[macro_use] extern crate euler;
/// # use euler::Quat;
/// # fn main() {
/// use std::f32::consts::PI;
/// let q = quat!(1.0, 0.0, 0.0; PI / 2.0);
/// assert_relative_eq!(q, Quat::new(f32::cos(PI / 4.0), 0.0, 0.0, f32::sin(PI / 4.0)));
/// # }
/// ```
///
/// Rotation around a `Vec3` axis.
///
/// ```rust
/// # #[macro_use] extern crate approx;
/// # #[macro_use] extern crate euler;
/// # use euler::Quat;
/// # fn main() {
/// use std::f32::consts::PI;
/// let axis = vec3!(1.0, 0.0, 0.0);
/// let q = quat!(axis; PI / 2.0);
/// assert_relative_eq!(q, Quat::new(f32::cos(PI / 4.0), 0.0, 0.0, f32::sin(PI / 4.0)));
/// # }
/// ```
#[macro_export]
macro_rules! quat {
    () => {
        $crate::Quat::identity()
    };

    ($expr:expr) => {
        $crate::Quat::from($expr)
    };

    ($axis:expr; $angle:expr) => {
        $crate::Quat::axis_angle($axis, $angle as f32)
    };

    ($x:expr, $y:expr, $z:expr; $angle:expr) => {
        $crate::Quat::axis_angle(
            $crate::Vec3::new($x as f32, $y as f32, $z as f32),
            $angle as f32,
        )
    };
}

/// Double-precision quaternion macro constructor.
///
/// Identity.
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let q = dquat!();
/// assert_eq!(q.as_ref(), &[0.0, 0.0, 0.0, 1.0]);
/// # }
/// ```
///
/// Rotation around explicit axis values.
///
/// ```rust
/// # #[macro_use] extern crate approx;
/// # #[macro_use] extern crate euler;
/// # use euler::DQuat;
/// # fn main() {
/// use std::f64::consts::PI;
/// let q = dquat!(1.0, 0.0, 0.0; PI / 2.0);
/// assert_relative_eq!(q, DQuat::new(f64::cos(PI / 4.0), 0.0, 0.0, f64::sin(PI / 4.0)));
/// # }
/// ```
///
/// Rotation around a `DVec3` axis.
///
/// ```rust
/// # #[macro_use] extern crate approx;
/// # #[macro_use] extern crate euler;
/// # use euler::DQuat;
/// # fn main() {
/// use std::f64::consts::PI;
/// let axis = dvec3!(1.0, 0.0, 0.0);
/// let q = dquat!(axis; PI / 2.0);
/// assert_relative_eq!(q, DQuat::new(f64::cos(PI / 4.0), 0.0, 0.0, f64::sin(PI / 4.0)));
/// # }
/// ```
#[macro_export]
macro_rules! dquat {
    () => {
        $crate::DQuat::identity()
    };

    ($expr:expr) => {
        $crate::DQuat::from($expr)
    };

    ($axis:expr; $angle:expr) => {
        $crate::DQuat::axis_angle($axis, $angle as f64)
    };

    ($x:expr, $y:expr, $z:expr; $angle:expr) => {
        $crate::DQuat::axis_angle(
            $crate::DVec3::new($x as f64, $y as f64, $z as f64),
            $angle as f64,
        )
    };
}

/// Single-precision 2D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = vec2!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = vec2!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = vec2!(1.2, 3.4);
/// assert_eq!(full.as_ref(), &[1.2, 3.4]);
/// # }
/// ```
#[macro_export]
macro_rules! vec2 {
    () => {
        $crate::Vec2::default()
    };

    ($expr:expr) => {
        $crate::Vec2::from($expr)
    };

    ($x:expr, $y:expr) => {
        $crate::Vec2::new($x as f32, $y as f32)
    };
}

/// Double-precision 2D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = dvec2!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = dvec2!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dvec2!(1.2, 3.4);
/// assert_eq!(full.as_ref(), &[1.2, 3.4]);
/// # }
/// ```
#[macro_export]
macro_rules! dvec2 {
    () => {
        $crate::DVec2::default()
    };

    ($expr:expr) => {
        $crate::DVec2::from($expr)
    };

    ($x:expr, $y:expr) => {
        $crate::DVec2::new($x as f64, $y as f64)
    };
}

/// Single-precision 3D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = vec3!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = vec3!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = vec3!(1.2, 3.4, 5.6);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6]);
/// # }
/// ```
///
/// From `vec2`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec2 = vec2!(1.2, 3.4);
/// let vec3 = vec3!(vec2, 5.6);
/// assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6]);
/// # }
/// ```
#[macro_export]
macro_rules! vec3 {
    () => {
        $crate::Vec3::default()
    };

    ($expr:expr) => {
        $crate::Vec3::from($expr)
    };

    ($xy:expr, $z:expr) => {
        $crate::Vec3::from(($xy, $z as f32))
    };

    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vec3::new($x as f32, $y as f32, $z as f32)
    };
}

/// Double-precision 3D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = dvec3!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = dvec3!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dvec3!(1.2, 3.4, 5.6);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6]);
/// # }
/// ```
///
/// From `vec2`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec2 = dvec2!(1.2, 3.4);
/// let vec3 = dvec3!(vec2, 5.6);
/// assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6]);
/// # }
/// ```
#[macro_export]
macro_rules! dvec3 {
    () => {
        $crate::DVec3::default()
    };

    ($expr:expr) => {
        $crate::DVec3::from($expr)
    };

    ($xy:expr, $z:expr) => {
        $crate::DVec3::from(($xy, $z as f64))
    };

    ($x:expr, $y:expr, $z:expr) => {
        $crate::DVec3::new($x as f64, $y as f64, $z as f64)
    };
}

/// Single-precision 4D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = vec4!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = vec4!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = vec4!(1.2, 3.4, 5.6, 7.8);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
///
/// From `vec2`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec2 = vec2!(1.2, 3.4);
/// let vec4 = vec4!(vec2, 5.6, 7.8);
/// assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
///
/// From `vec3`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec3 = vec3!(1.2, 3.4, 5.6);
/// let vec4 = vec4!(vec3, 7.8);
/// assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
#[macro_export]
macro_rules! vec4 {
    () => {
        $crate::Vec4::default()
    };

    ($expr:expr) => {
        $crate::Vec4::from($expr)
    };

    ($xyz:expr, $w:expr) => {
        $crate::Vec4::from(($xyz, $w as f32))
    };

    ($xy:expr, $z:expr, $w:expr) => {
        $crate::Vec4::from(($xy, $z as f32, $w as f32))
    };

    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::Vec4::new($x as f32, $y as f32, $z as f32, $w as f32)
    };
}

/// Double-precision 4D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let zeros = dvec4!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0, 0.0]);
/// # }
/// ```
///
/// Ones
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let ones = dvec4!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0, 1.0]);
/// # }
/// ```
///
/// Full
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let full = dvec4!(1.2, 3.4, 5.6, 7.8);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
///
/// From `vec2`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec2 = dvec2!(1.2, 3.4);
/// let vec4 = dvec4!(vec2, 5.6, 7.8);
/// assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
///
/// From `vec3`
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// let vec3 = dvec3!(1.2, 3.4, 5.6);
/// let vec4 = dvec4!(vec3, 7.8);
/// assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// # }
/// ```
#[macro_export]
macro_rules! dvec4 {
    () => {
        $crate::DVec4::default()
    };

    ($expr:expr) => {
        $crate::DVec4::from($expr)
    };

    ($xyz:expr, $w:expr) => {
        $crate::DVec4::from(($xyz, $w as f64))
    };

    ($xy:expr, $z:expr, $w:expr) => {
        $crate::DVec4::from(($xy, $z as f64, $w as f64))
    };

    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::DVec4::new($x as f64, $y as f64, $z as f64, $w as f64)
    };
}
