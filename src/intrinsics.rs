use core::mem;

// arithmetic

mod add;
mod div;
mod mul;
mod rem;
mod sub;

pub use add::simd_add;
pub use div::simd_div;
pub use mul::simd_mul;
pub use rem::simd_rem;
pub use sub::simd_sub;

// cast

mod cast;

pub use cast::{simd_cast, simd_cast_mask};

// use an identity (one, zero)

mod product;
mod sum;

pub use product::simd_product;
pub use sum::simd_sum;

// bitwise ops

mod and;

pub use and::simd_and;

// logic ops

mod eq;
mod ge;
mod gt;
mod le;
mod lt;
mod ne;

pub use eq::simd_eq;
pub use ge::simd_ge;
pub use gt::simd_gt;
pub use le::simd_le;
pub use lt::simd_lt;
pub use ne::simd_ne;

#[derive(Clone, Copy, Debug)]
#[repr(simd)]
struct Simd<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Simd<T, N> {
    /// Converts an array to a SIMD vector.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    /// Converts a SIMD vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; N]
    where
        T: Copy,
    {
        unsafe { mem::transmute_copy(&self) }
    }
}
