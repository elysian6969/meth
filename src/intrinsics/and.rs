use super::Simd;
use core::ops::BitAnd;

#[inline]
#[must_use]
pub unsafe fn simd_and<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: BitAnd,
{
    extern "platform-intrinsic" {
        fn simd_and<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_and(a, b).to_array()
}
