use super::Simd;
use crate::identity::One;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Mul;

/// Product of a SIMD vector.
#[inline]
pub const fn simd_product<T, const LANES: usize>(a: Simd<T, LANES>) -> T
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    T: ~const One,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD product, scalar, compile-time.
    #[inline]
    pub const fn scalar_product<T, const LANES: usize>(a: Simd<T, LANES>) -> T
    where
        T: ~const Element,
        T: ~const Mul<Output = T>,
        T: ~const One,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = One::one();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result = result * a.get(i);
            }

            i += 1;
        }

        result
    }

    /// SIMD product, intrinsic, runtime.
    #[inline]
    pub fn simd_product<T, const LANES: usize>(a: Simd<T, LANES>) -> T
    where
        T: Element,
        T: Mul<Output = T>,
        T: One,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_reduce_mul_ordered<T, U>(a: T, b: U) -> U;
        }

        unsafe { simd_reduce_mul_ordered(a, One::one()) }
    }

    unsafe { const_eval_select((a,), scalar_product, simd_product) }
}
