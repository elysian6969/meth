use super::Simd;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Sub;

/// Subtract two SIMD vectors.
#[inline]
pub const fn simd_sub<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD subtract, scalar, compile-time.
    #[inline]
    pub const fn scalar_sub<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Sub<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            // SAFETY: Index is valid.
            unsafe {
                result.set(i, a.get(i) - b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD subtract, intrinsic, runtime.
    #[inline]
    pub fn simd_sub<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Sub<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_sub<T>(a: T, b: T) -> T;
        }

        // SAFETY: Where clauses (should) validate input.
        unsafe { simd_sub(a, b) }
    }

    // SAFETY: Both functions (should) produce the same results.
    unsafe { const_eval_select((a, b), scalar_sub, simd_sub) }
}
