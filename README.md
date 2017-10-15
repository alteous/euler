# euler

A mathematics library for 3D computer graphics.

## Quick facts

* Wrapper around `cgmath`, solving the most common tasks quickly.
* Specifically for 3D computer graphics.
* Layout and handedness follows OpenGL conventions.
* All angles are assumed to be in radians.
* By design, only `f32` and `f64` bases are supported.
* No distinction between 'point' and 'vector' types.

## Demonstrations

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

## Rationale

### Why create yet another mathematics library?

The creation of `euler` is a reaction to the common design choices of popular existing libraries such as `cgmath`, `euclid`, and `nalgebra`. The author believes that, although these libraries are suitable for a wide range of use cases, they are not nearly simple enough for application code where vector and matrix code is very common such as 3D games. The author believes these libraries suffer from 'genericitis' (plagued with many layers of abstraction and/or generics) to the point of being impractical for quick prototyping.

## Objectives

The author wishes to provide a library that:

* Allows code for mathematics to be written declaratively.
* Keeps generics to a minimum.
* Does not require traits or prelude modules to be imported for any functionality.
* Enables easy and intuitive conversions between differently size types, for example `vec2` to `vec4` and `mat4` to `mat3`.
* Allows straightforward conversion between types of different precision, for example `dvec2` to/from `vec2`.

The author intends to achieve the above objectives by mimicking GLSL syntax through the Rust macro system. The `euler` macro syntax is designed to capture the most common usages of GLSL vector and matrix constructors; however, it is not designed to match exactly the syntax of GLSL.

## Q & A

The following are answers to questions and comments that have been asked by members of the rust-gamedev community.

### Wait, you're using *macros* for this‽

Yes, since Rust doesn't allow function overloading. One may use `euler` without the macros but that would defeat the point!

### Does this not bypass the module system?

That's correct. This is a side-effect of using macros in their current state, however in applications such as 3D games where vector and matrix code is so common, it is an unnecessary inconvenience to have to import preludes, traits, and type aliases for every module. If this rings alarm bells in your ears then perhaps `euler` isn't for you.

### Does using macros increase compile times?

Maybe, but I personally haven't noticed. Since the library performs only shallow macro expansions (usually a single function call) and there are very few generic code, compile times are often fast.

### Without a point type, how you do distinguish between position and direction vector transforms?

The library's stance is that transforming vectors should be explicit through the mathematics and not through the type system. Therefore, when transforming 3D vectors, users are encouraged to multiply using `vec4!(xyz, 1)` for position vectors and `vec4!(xyz, 0)` for direction vectors explicitly.

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
