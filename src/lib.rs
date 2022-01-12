#![allow(incomplete_features)]
#![feature(const_eval_select)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_trait_bound)]
#![feature(const_intrinsic_copy)]
#![feature(const_mut_refs)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_read)]
#![feature(const_ptr_write)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(maybe_uninit_uninit_array)]
#![feature(platform_intrinsics)]
#![feature(portable_simd)]
#![feature(repr_simd)]
#![no_std]

pub use self::euler_angles::EulerAngles;
pub use self::matrix::Matrix;
pub use self::quaternion::Quaternion;
pub use self::real::Real;
pub use self::vec::{Element, Lanes, SupportedLanes, Vec};

mod euler_angles;
mod matrix;
mod ops;
mod quaternion;
mod real;
mod vec;

pub mod identity;

pub(crate) mod mem;
