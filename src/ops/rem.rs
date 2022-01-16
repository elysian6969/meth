use super::Simd;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Rem;

/// Remainder of two SIMD vectors.
#[inline]
pub const fn simd_rem<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD remainder, scalar, compile-time.
    #[inline]
    pub const fn scalar_rem<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Rem<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) % b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD remainder, intrinsic, runtime.
    #[inline]
    pub fn simd_rem<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Rem<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_rem<T>(a: T, b: T) -> T;
        }

        unsafe { simd_rem(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_rem, simd_rem) }
}
