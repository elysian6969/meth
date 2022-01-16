use core::marker::PhantomData;
use core::mem;

mod sealed {
    pub trait Sealed {}
}

/// SIMD lane count.
pub struct Lanes<T, const N: usize>(PhantomData<[T; N]>);

/// SIMD-able lanes.
pub trait LaneCount {
    const LANES: usize;
}

/// Determine the nearest lane to `lanes` for the given `size`.
const fn nearest_lane(size: usize, lanes: usize) -> usize {
    let max_lanes = 32
        / if size < 8 {
            size
        } else {
            panic!("unsupported type");
        };

    let lanes = if lanes.is_power_of_two() {
        lanes
    } else {
        match lanes.checked_next_power_of_two() {
            Some(lanes) => lanes >> 1,
            None => return max_lanes,
        }
    };

    if lanes < 2 {
        0
    } else if lanes < max_lanes {
        lanes
    } else {
        max_lanes
    }
}

macro_rules! impl_lanes {
    { $ty:ty } => {
        impl<const N: usize> const sealed::Sealed for Lanes<$ty, N> {}

        impl<const N: usize> const LaneCount for Lanes<$ty, N> {
            const LANES: usize = nearest_lane(mem::size_of::<$ty>(), N);
        }
    }
}

impl_lanes! { f32 }
impl_lanes! { f64 }

impl_lanes! { i8 }
impl_lanes! { i16 }
impl_lanes! { i32 }
impl_lanes! { i64 }
impl_lanes! { isize }

impl_lanes! { u8 }
impl_lanes! { u16 }
impl_lanes! { u32 }
impl_lanes! { u64 }
impl_lanes! { usize }
