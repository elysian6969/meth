use super::Simd;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Div;

/// Divide two SIMD vectors.
#[inline]
pub const fn simd_div<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD divide, scalar, compile-time.
    #[inline]
    pub const fn scalar_div<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Div<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) / b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD divide, intrinsic, runtime.
    #[inline]
    pub fn simd_div<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Div<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_div<T>(a: T, b: T) -> T;
        }

        unsafe { simd_div(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_div, simd_div) }
}
