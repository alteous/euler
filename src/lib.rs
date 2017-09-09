#[macro_use]
extern crate approx;
extern crate cgmath;

#[macro_use]
mod macros;

mod mat2;
mod mat3;
mod mat4;
mod quat;
mod transform3;
mod vec2;
mod vec3;
mod vec4;

pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;
pub use quat::Quat;
pub use transform3::Transform3;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

#[cfg(test)]
mod tests {
    #[test]
    fn mat2_macro_empty() {
        let empty = mat2!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0],
                [0.0, 1.0],
            ]
        );
    }

    #[test]
    fn mat2_macro_single() {
        let single = mat2!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0],
                [0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat2_macro_full() {
        let full = mat2!(
            1.2, 3.4,
            5.6, 7.8,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [1.2, 3.4],
                [5.6, 7.8],
            ]
        );
    }

    #[test]
    fn mat3_macro_empty() {
        let empty = mat3!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        );
    }
    #[test]
    fn mat3_macro_single() {
        let single = mat3!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
                [0.0, 0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat3_macro_full() {
        let full = mat3!(
            0.1, 0.2, 0.3,
            0.4, 0.5, 0.6,
            0.7, 0.8, 0.9,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [0.1, 0.2, 0.3],
                [0.4, 0.5, 0.6],
                [0.7, 0.8, 0.9],
            ]
        );
    }

    #[test]
    fn mat4_macro_empty() {
        let empty = mat4!();
        assert_eq!(
            empty.as_ref(),
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn mat4_macro_single() {
        let single = mat4!(2.0);
        assert_eq!(
            single.as_ref(),
            &[
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ]
        );
    }

    #[test]
    fn mat4_macro_full() {
        let full = mat4!(
            0.1, 0.2, 0.3, 0.4,
            0.5, 0.6, 0.7, 0.8,
            0.9, 1.0, 1.1, 1.2,
            1.3, 1.4, 1.5, 1.6,
        );
        assert_eq!(
            full.as_ref(),
            &[
                [0.1, 0.2, 0.3, 0.4],
                [0.5, 0.6, 0.7, 0.8],
                [0.9, 1.0, 1.1, 1.2],
                [1.3, 1.4, 1.5, 1.6],
            ]
        );
    }

    #[test]
    fn vec2_relative_eq() {
        assert_relative_eq!(vec2!(1.0, 2.0), vec2!(1.0, 2.0));
        assert_relative_ne!(vec2!(1.0, 2.0), vec2!(1.0, 2.1));
    }

    #[test]
    fn vec2_plus_scalar() {
        assert_relative_eq!(vec2!(2.2, 4.4), vec2!(1.2, 3.4) + 1.0);
    }

    #[test]
    fn vec2_minus_scalar() {
        assert_relative_eq!(vec2!(0.2, 2.4), vec2!(2.2, 4.4) - 2.0);
    }

    #[test]
    fn vec2_plus_vec2() {
        assert_relative_eq!(vec2!(1.6, 4.0), vec2!(1.2, 3.4) + vec2!(0.4, 0.6));
    }

    #[test]
    fn vec2_minus_vec2() {
        assert_relative_eq!(vec2!(), vec2!(2.2, 4.4) - vec2!(2.2, 4.4));
    }

    #[test]
    fn vec3_relative_eq() {
        assert_relative_eq!(vec3!(1.0, 2.0, 3.0), vec3!(1.0, 2.0, 3.0));
        assert_relative_ne!(vec3!(1.0, 2.0, 3.0), vec3!(1.0, 2.0, 3.1));
    }

    #[test]
    fn vec3_plus_scalar() {
        assert_relative_eq!(vec3!(2.2, 4.4, 6.6), vec3!(1.2, 3.4, 5.6) + 1.0);
    }

    #[test]
    fn vec3_minus_scalar() {
        assert_relative_eq!(vec3!(0.2, 2.4, 4.6), vec3!(1.2, 3.4, 5.6) - 1.0);
    }

    #[test]
    fn vec3_plus_vec3() {
        assert_relative_eq!(
            vec3!(2.2, 5.4, 8.6),
            vec3!(1.2, 3.4, 5.6) + vec3!(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn vec3_minus_vec3() {
        assert_relative_eq!(
            vec3!(0.2, 1.4, 2.6),
            vec3!(1.2, 3.4, 5.6) - vec3!(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn vec2_macro_empty() {
        let zero = vec2!();
        assert_eq!(zero.as_ref(), &[0.0, 0.0]);
    }    

    #[test]
    fn vec2_macro_ones() {
        let ones = vec2!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0]);
    }

    #[test]
    fn vec2_macro_full() {
        let full = vec2!(1.2, 3.4);
        assert_eq!(full.as_ref(), &[1.2, 3.4]);
    }

    #[test]
    fn vec3_macro_zeros() {
        let zeros = vec3!();
        assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0]);
        assert_eq!(zeros.w, 0.0);
    }

    #[test]
    fn vec3_macro_ones() {
        let ones = vec3!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0]);
        assert_eq!(ones.w, 0.0);
    }

    #[test]
    fn vec3_macro_full() {
        let full = vec3!(1.2, 3.4, 5.6);
        assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6]);
        
    }

    #[test]
    fn vec3_macro_from_vec2() {
        let vec2 = vec2!(1.2, 3.4);
        let vec3 = vec3!(vec2, 5.6);
        assert_eq!(vec3.as_ref(), &[1.2, 3.4, 5.6]);
        assert_eq!(vec3.w, 0.0);
    }

    #[test]
    fn vec4_macro_zeros() {
        let zeros = vec4!();
        assert_eq!(zeros.as_ref(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn vec4_macro_ones() {
        let ones = vec4!(1.0);
        assert_eq!(ones.as_ref(), &[1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn vec4_macro_full() {
        let full = vec4!(1.2, 3.4, 5.6, 7.8);
        assert_eq!(full.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_macro_from_vec2() {
        let vec2 = vec2!(1.2, 3.4);
        let vec4 = vec4!(vec2, 5.6, 7.8);
        assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_macro_from_vec3() {
        let vec3 = vec3!(1.2, 3.4, 5.6);
        let vec4 = vec4!(vec3, 7.8);
        assert_eq!(vec4.as_ref(), &[1.2, 3.4, 5.6, 7.8]);
    }

    #[test]
    fn vec4_relative_eq() {
        assert_relative_eq!(vec4!(1.0, 2.0, 3.0, 1.0), vec4!(1.0, 2.0, 3.0, 1.0));
        assert_relative_ne!(vec4!(1.0, 2.0, 3.0, 1.0), vec4!(1.0, 2.0, 3.0, 0.0));
    }
}
