/// 2x2 macro constructor.
///
/// # Examples
///
/// Identity
///
/// ```rust,ignore
/// let identity = mat2!();
/// assert_eq!(identity.as_ref(), &[1.0, 0.0, 1.0, 0.0]);
/// ```
#[macro_export]
macro_rules! mat2 {
    () => {
        $crate::Mat2 {
            m00: 1.0, m01: 0.0,
            m10: 0.0, m11: 1.0,
        }
    };

    ($xx:expr) => {
        $crate::Mat2 {
            m00: $xx, m01: 0.0,
            m10: 0.0, m11: $xx,
        }
    };

    (
        $m00:expr, $m01:expr,
        $m10:expr, $m11:expr,
    ) => {
        $crate::Mat2 {
            m00: $m00, m01: $m01,
            m10: $m10, m11: $m11,
        }
    };
}

#[macro_export]
macro_rules! mat3 {
    () => {
        $crate::Mat3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    };

    ($xx:expr) => {
        $crate::Mat3 {
            m00: $xx, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: $xx, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: $xx,
        }
    };

    (
        $m00:expr, $m01:expr, $m02:expr,
        $m10:expr, $m11:expr, $m12:expr,
        $m20:expr, $m21:expr, $m22:expr,
    ) => {
        $crate::Mat3 {
            m00: $m00, m01: $m01, m02: $m02,
            m10: $m10, m11: $m11, m12: $m12,
            m20: $m20, m21: $m21, m22: $m22,
        }
    };
}

#[macro_export]
macro_rules! mat4 {
    () => {
        $crate::Mat4 {
            m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
        }
    };

    ($xx:expr) => {
        $crate::Mat4 {
            m00: $xx, m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: $xx, m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: $xx, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: $xx,
        }
    };

    (
        $m00:expr, $m01:expr, $m02:expr, $m03:expr,
        $m10:expr, $m11:expr, $m12:expr, $m13:expr,
        $m20:expr, $m21:expr, $m22:expr, $m23:expr,
        $m30:expr, $m31:expr, $m32:expr, $m33:expr,
    ) => {
        $crate::Mat4 {
            m00: $m00, m01: $m01, m02: $m02, m03: $m03,
            m10: $m10, m11: $m11, m12: $m12, m13: $m13,
            m20: $m20, m21: $m21, m22: $m22, m23: $m23,
            m30: $m30, m31: $m31, m32: $m32, m33: $m33,
        }
    };
}

/// 2D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust,ignore
/// let zeros = vec2!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0]);
/// ```
///
/// Ones
///
/// ```rust,ignore
/// let ones = vec2!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0]);
/// ```
///
/// Full
///
/// ```rust,ignore
/// let full = vec2!(1.2, 3.4);
/// assert_eq!(full.as_ref(), &[1.2, 3.4]);
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
/// ```rust,ignore
/// let zeros = vec3!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0]);
/// ```
///
/// Ones
///
/// ```rust,ignore
/// let ones = vec3!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0]);
/// ```
///
/// Full
///
/// ```rust,ignore
/// let full = vec3!(1.2, 3.4, 5.6);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6]);
/// ```
///
/// From `vec2`
///
/// ```rust,ignore
/// let vec2 = vec2!(1.2, 3.4);
/// let vec3 = vec3!(vec2, 5.6);
/// assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6]);
/// ```
#[macro_export]
macro_rules! vec3 {
    () => {
        $crate::Vec3 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    };

    ($xyz:expr) => {
        $crate::Vec3 { x: $xyz, y: $xyz, z: $xyz, w: 0.0 }
    };
    
    ($xy:expr, $z:expr) => {
        $crate::Vec3 {
            x: $crate::Vec2::from($xy).x,
            y: $crate::Vec2::from($xy).y,
            z: $z,
            w: 0.0,
        }
    };

    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vec3 { x: $x, y: $y, z: $z, w: 0.0 }
    };
}

/// Homogeneous 3D vector macro constructor.
///
/// # Examples
///
/// Zeros
///
/// ```rust,ignore
/// let zeros = vec4!();
/// assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0, 0.0]);
/// ```
///
/// Ones
///
/// ```rust,ignore
/// let ones = vec4!(1.0);
/// assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0, 1.0]);
/// ```
///
/// Full
///
/// ```rust,ignore
/// let full = vec4!(1.2, 3.4, 5.6, 7.8);
/// assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// ```
///
/// From `vec2`
///
/// ```rust,ignore
/// let vec2 = vec2!(1.2, 3.4);
/// let vec4 = vec4!(vec2, 5.6, 7.8);
/// assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
/// ```
///
/// From `vec3`
///
/// ```rust,ignore
/// let vec3 = vec3!(1.2, 3.4, 5.6);
/// let vec4 = vec4!(vec3, 7.8);
/// assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
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
