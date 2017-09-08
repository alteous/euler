
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
