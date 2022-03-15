use super::{simd_cast_mask, Simd};
use core::cmp::Eq;
use core::ops::BitAnd;

#[inline]
#[must_use]
pub unsafe fn simd_eq<T, const N: usize>(a: [T; N], b: [T; N]) -> [bool; N]
where
    T: Copy,
    T: Eq,
    T: BitAnd,
{
    extern "platform-intrinsic" {
        fn simd_eq<T, U>(a: T, b: T) -> U;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);
    let c = Simd::<T, N>::to_array(simd_eq(a, b));

    simd_cast_mask(c)
}
