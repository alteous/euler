# Changelog

Notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

The `euler` crate adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2017-09-09

### Added

- Vector types `Vec2`, `Vec3`, and `Vec4`.
- Matrix types `Mat2`, `Mat3`, and `Mat4`.
- Quaternion type `Quat`.
- Vector macro constructors `vec2!`, `vec3!`, and `vec4!`.
- Matrix macro constructors `mat2!`, `mat3!`, and `mat4!`.
- Quaternion macro constructor `quat!`.
- Transform type, representing translation * rotation * non-uniform scale.
- Vector and matrix multiplication operators:
  - `Mat2 * Mat2`
  - `Mat2 * Vec2`
  - `Mat3 * Mat3`
  - `Mat3 * Vec3`
  - `Mat4 * Mat4`
  - `Mat4 * Vec4`
- Vector addition operators:
  - `Vec2 + Vec2`
  - `Vec3 + Vec3`
- Functions `invert` and `try_invert` implemented for all matrix types.
