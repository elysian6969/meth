#![allow(incomplete_features)]
#![deny(warnings)]
#![feature(const_eval_select)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_ptr_read)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(generic_const_exprs)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![no_std]

pub use euler_angles::EulerAngles;
pub use matrix::Matrix;
pub use quaternion::Quaternion;
pub use real::Real;
pub use vec::{Element, LaneCount, Lanes, Vec};
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

mod euler_angles;
mod matrix;
mod quaternion;
mod real;
mod vec2;
mod vec3;
mod vec4;

pub mod identity;
pub mod intrinsics;
pub mod vec;
