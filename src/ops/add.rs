use super::Simd;
use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::Add;

/// Add two SIMD vectors.
#[inline]
pub const fn simd_add<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD add, scalar, compile-time.
    #[inline]
    pub const fn scalar_add<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Add<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) + b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD add, intrinsic, runtime.
    #[inline]
    pub fn simd_add<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Add<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_add<T>(a: T, b: T) -> T;
        }

        unsafe { simd_add(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_add, simd_add) }
}
