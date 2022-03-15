use super::{simd_and, Simd};
use core::mem;
use core::ops::BitAnd;

#[inline]
#[must_use]
pub unsafe fn simd_cast<T, U, const N: usize>(a: [T; N]) -> [U; N]
where
    T: Copy,
    U: Copy,
{
    extern "platform-intrinsic" {
        fn simd_cast<T, U>(x: T) -> U;
    }

    let a = Simd::from_array(a);
    let b = simd_cast(a);

    Simd::<U, N>::to_array(b)
}

#[inline]
#[must_use]
pub unsafe fn simd_cast_mask<T, const N: usize>(a: [T; N]) -> [bool; N]
where
    T: Copy,
    T: BitAnd,
{
    let c: [i8; N] = simd_cast(a);
    let c = simd_and(c, [1; N]);

    mem::transmute_copy(&c)
}
