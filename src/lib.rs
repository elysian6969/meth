#![allow(incomplete_features)]
#![deny(warnings)]
#![feature(const_eval_select)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_trait_bound)]
#![feature(const_intrinsic_copy)]
#![feature(const_maybe_uninit_as_mut_ptr)]
#![feature(const_mut_refs)]
#![feature(const_ptr_offset)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![no_std]

pub use euler_angles::EulerAngles;
pub use matrix::Matrix;
pub use quaternion::Quaternion;
pub use real::Real;
pub use vec::{Element, LaneCount, Lanes, Vec};

mod euler_angles;
mod intrinsics;
mod matrix;
mod quaternion;
mod real;

pub mod identity;
pub mod vec;
