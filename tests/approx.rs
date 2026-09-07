use approx::{
    assert_abs_diff_eq, assert_abs_diff_ne, assert_relative_eq, assert_relative_ne, assert_ulps_eq,
    assert_ulps_ne,
};
use euler::*;

// Exercise each comparison's tolerance independently through approx's public macros.
macro_rules! comparison_test {
    ($name:ident, $scalar:ty, $make:expr) => {
        #[test]
        fn $name() {
            let make = $make;
            let a = make(1000.0);
            let b = make(1000.125);
            assert_abs_diff_eq!(a, b, epsilon = 0.25);
            assert_abs_diff_ne!(a, b, epsilon = 0.0625);
            assert_relative_eq!(a, b, epsilon = 0.0, max_relative = 0.001);
            assert_relative_ne!(a, b, epsilon = 0.0, max_relative = 0.00001);

            let a = make(1.0);
            let b = make(<$scalar>::from_bits((1.0 as $scalar).to_bits() + 2));
            assert_ulps_eq!(a, b, epsilon = 0.0, max_ulps = 2);
            assert_ulps_ne!(a, b, epsilon = 0.0, max_ulps = 1);
            assert_abs_diff_eq!(a, a);
            assert_relative_eq!(a, a);
            assert_ulps_eq!(a, a);
        }
    };
}

comparison_test!(vec2, f32, |x| Vec2::from([x; 2]));
comparison_test!(vec3, f32, |x| Vec3::from([x; 3]));
comparison_test!(vec4, f32, |x| Vec4::from([x; 4]));
comparison_test!(dvec2, f64, |x| DVec2::from([x; 2]));
comparison_test!(dvec3, f64, |x| DVec3::from([x; 3]));
comparison_test!(dvec4, f64, |x| DVec4::from([x; 4]));
comparison_test!(mat2, f32, |x| Mat2::from([[x; 2]; 2]));
comparison_test!(mat3, f32, |x| Mat3::from([[x; 3]; 3]));
comparison_test!(mat4, f32, |x| Mat4::from([[x; 4]; 4]));
comparison_test!(dmat2, f64, |x| DMat2::from([[x; 2]; 2]));
comparison_test!(dmat3, f64, |x| DMat3::from([[x; 3]; 3]));
comparison_test!(dmat4, f64, |x| DMat4::from([[x; 4]; 4]));
comparison_test!(quat, f32, |x| Quat::from([x; 4]));
comparison_test!(dquat, f64, |x| DQuat::from([x; 4]));

// Vary translation, rotation, and scale separately so no field can be skipped.
comparison_test!(trs_translation, f32, |x| Trs {
    t: Vec3::from([x; 3]),
    ..Trs::identity()
});
comparison_test!(trs_rotation, f32, |x| Trs {
    r: Quat::from([x; 4]),
    ..Trs::identity()
});
comparison_test!(trs_scale, f32, |x| Trs {
    s: Vec3::from([x; 3]),
    ..Trs::identity()
});
comparison_test!(dtrs_translation, f64, |x| DTrs {
    t: DVec3::from([x; 3]),
    ..DTrs::identity()
});
comparison_test!(dtrs_rotation, f64, |x| DTrs {
    r: DQuat::from([x; 4]),
    ..DTrs::identity()
});
comparison_test!(dtrs_scale, f64, |x| DTrs {
    s: DVec3::from([x; 3]),
    ..DTrs::identity()
});
