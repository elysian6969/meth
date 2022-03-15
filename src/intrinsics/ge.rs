use super::{simd_cast_mask, Simd};
use core::cmp::PartialOrd;
use core::ops::BitAnd;

#[inline]
#[must_use]
pub unsafe fn simd_ge<T, const N: usize>(a: [T; N], b: [T; N]) -> [bool; N]
where
    T: Copy,
    T: PartialOrd,
    T: BitAnd,
{
    extern "platform-intrinsic" {
        fn simd_ge<T, U>(a: T, b: T) -> U;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);
    let c = Simd::<T, N>::to_array(simd_ge(a, b));

    simd_cast_mask(c)
}
