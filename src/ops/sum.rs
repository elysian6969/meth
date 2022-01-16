use super::Simd;
use crate::identity::Zero;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Add;

/// Sum of a SIMD vector.
#[inline]
pub const fn simd_sum<T, const LANES: usize>(a: Simd<T, LANES>) -> T
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    T: ~const Zero,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD sum, scalar, compile-time.
    #[inline]
    pub const fn scalar_sum<T, const LANES: usize>(a: Simd<T, LANES>) -> T
    where
        T: ~const Element,
        T: ~const Add<Output = T>,
        T: ~const Zero,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Zero::zero();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result = result + a.get(i);
            }

            i += 1;
        }

        result
    }

    /// SIMD sum, intrinsic, runtime.
    #[inline]
    pub fn simd_sum<T, const LANES: usize>(a: Simd<T, LANES>) -> T
    where
        T: Element,
        T: Add<Output = T>,
        T: Zero,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_reduce_add_ordered<T, U>(a: T, b: U) -> U;
        }

        unsafe { simd_reduce_add_ordered(a, Zero::zero()) }
    }

    unsafe { const_eval_select((a,), scalar_sum, simd_sum) }
}
