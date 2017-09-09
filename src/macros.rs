/// 2x2 matrix macro constructor.
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

    (
        $m00:expr, $m01:expr,
        $m10:expr, $m11:expr,
    ) => {
        $crate::Mat2::from([
            [$m00, $m01],
            [$m10, $m11],
        ])
    };
}

/// 3x3 matrix macro constructor.
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

    (
        $m00:expr, $m01:expr, $m02:expr,
        $m10:expr, $m11:expr, $m12:expr,
        $m20:expr, $m21:expr, $m22:expr,
    ) => {
        $crate::Mat3::from([
            [$m00, $m01, $m02],
            [$m10, $m11, $m12],
            [$m20, $m21, $m22],
        ])
    };
}

/// 4x4 matrix macro constructor.
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

    (
        $m00:expr, $m01:expr, $m02:expr, $m03:expr,
        $m10:expr, $m11:expr, $m12:expr, $m13:expr,
        $m20:expr, $m21:expr, $m22:expr, $m23:expr,
        $m30:expr, $m31:expr, $m32:expr, $m33:expr,
    ) => {
        $crate::Mat4::from([
            [$m00, $m01, $m02, $m03],
            [$m10, $m11, $m12, $m13],
            [$m20, $m21, $m22, $m23],
            [$m30, $m31, $m32, $m33],
        ])
    };
}

/// Quaternion macro constructor.
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
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// use std::f32::consts::PI;
/// let q = quat!(1.0, 0.0, 0.0, PI / 2.0);
/// assert_eq!(q.as_ref(), &[f32::cos(PI / 4.0), 0.0, 0.0, f32::sin(PI / 4.0)]);
/// # }
/// ```
///
/// Rotation around a `Vec3` axis.
///
/// ```rust
/// # #[macro_use] extern crate euler;
/// # fn main() {
/// use std::f32::consts::PI;
/// let axis = vec3!(1.0, 0.0, 0.0);
/// let q = quat!(axis, PI / 2.0);
/// assert_eq!(q.as_ref(), &[f32::cos(PI / 4.0), 0.0, 0.0, f32::sin(PI / 4.0)]);
/// # }
/// ```
#[macro_export]
macro_rules! quat {
    () => {
        $crate::Quat::identity()
    };

    ($xyz:expr, $r:expr) => {
        $crate::Quat::rotation_about_axis($xyz, $r)
    };

    ($x:expr, $y:expr, $z:expr, $r:expr) => {
        quat!(vec3!($x, $y, $z), $r)
    };
}

/// 2D vector macro constructor.
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
        $crate::Vec2 { x: 0.0, y: 0.0 }
    };

    ($x:expr) => {
        $crate::Vec2 { x: $x, y: $x }
    };

    ($x:expr, $y:expr) => {
        $crate::Vec2 { x: $x, y: $y }
    };
}

/// 3D vector macro constructor.
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
        $crate::Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    };

    ($xyz:expr) => {
        $crate::Vec3 { x: $xyz, y: $xyz, z: $xyz }
    };
    
    ($xy:expr, $z:expr) => {
        $crate::Vec3 {
            x: $crate::Vec2::from($xy).x,
            y: $crate::Vec2::from($xy).y,
            z: $z,
        }
    };

    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vec3 { x: $x, y: $y, z: $z }
    };
}

/// Homogeneous 3D vector macro constructor.
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
        $crate::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    };

    ($xyzw:expr) => {
        $crate::Vec4 { x: $xyzw, y: $xyzw, z: $xyzw, w: $xyzw }
    };

    ($xyz:expr, $w:expr) => {
        $crate::Vec4 {
            x: $crate::Vec3::from($xyz).x,
            y: $crate::Vec3::from($xyz).y,
            z: $crate::Vec3::from($xyz).z,
            w: $w,
        }
    };

    ($xy:expr, $z:expr, $w:expr) => {
        $crate::Vec4 {
            x: $crate::Vec2::from($xy).x,
            y: $crate::Vec2::from($xy).y,
            z: $z,
            w: $w,
        }
    };

    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::Vec4 { x: $x, y: $y, z: $z, w: $w }
    };
}
