#[macro_use]
extern crate euler;

use std::{f32, f64};

fn unproject_single_precision(ndc: euler::Vec2) -> euler::Vec3 {
    let projection = mat4!();
    let inverse_projection = projection.inverse();
    let eye = inverse_projection * vec4!(ndc, -1.0, 1.0);
    let view = euler::Trs::new(
        vec3!(1, 0, -1),
        quat!(1, 0, 0; f32::consts::PI / 2.0),
        vec3!(1.0),
    )
    .matrix();
    let inverse_view = view.inverse();
    let world = inverse_view * vec4!(eye.xy(), -1.0, 0.0);
    let ray = world.xyz().normalize();
    ray
}

fn unproject_double_precision(ndc: euler::Vec2) -> euler::DVec3 {
    let projection = dmat4!();
    let inverse_projection = projection.inverse();
    let eye = inverse_projection * dvec4!(dvec2!(ndc), -1, 1);
    let view = euler::DTrs::new(
        dvec3!(1, 0, -1),
        dquat!(1, 0, 0; f64::consts::PI / 2.0),
        dvec3!(1.0),
    )
    .matrix();
    let inverse_view = view.inverse();
    let world = inverse_view * dvec4!(eye.xy(), -1, 0);
    let ray = world.xyz().normalize();
    ray
}

fn main() {
    let ndc = vec2!(-0.5, 0.5);
    println!("{}", unproject_single_precision(ndc));
    println!("{}", unproject_double_precision(ndc));
}
