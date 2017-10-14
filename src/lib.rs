//! An experimental mathematics library for computer graphics.

#[allow(unused_imports)]
#[macro_use]
extern crate approx;
extern crate cgmath;
#[cfg(feature = "mint")]
extern crate mint;

#[macro_use]
mod macros;

mod mat;
mod quat;
mod trs;
mod vec;

pub use mat::{DMat2, DMat3, DMat4, Mat2, Mat3, Mat4};
pub use quat::{DQuat, Quat};
pub use trs::{DTrs, Trs};
pub use vec::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};
