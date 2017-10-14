# euler

An experimental mathematics library for computer graphics.

## Project ideas

 * Wrapper around `cgmath`, solving the most common tasks quickly.
 * Specifically for 3D computer graphics.
 * Layout and handedness follows OpenGL conventions.
 * No distinction between 'point' and 'vector' types.
 * No need to import traits or modules for most functionality.
 * GLSL-like constructors for vectors and matrices.
 * All angles are in radians.
 * ~~All types are base `f32`.~~ Only `f32` and `f64` bases are supported.

## Demonstration

#### Unprojecting a ray

```rust
let projection = mat4!();
let inverse_projection = projection.invert();
let ndc = vec2!(-0.5, 0.5);
let eye = inverse_projection * vec4!(ndc, -1, 1);
let view = euler::Trs::new(vec3!(1, 0, -1), quat!(1, 0, 0; std::f32::consts::PI / 2.0), vec3!(1.0)).matrix();
let inverse_view = view.invert();
let world = inverse_view * vec4!(eye.xy(), -1, 0);
let ray = world.xyz().normalize();
```

## License agreement

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
