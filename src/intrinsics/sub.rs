use super::Simd;
use core::ops::Sub;

#[inline]
#[must_use]
pub unsafe fn simd_sub<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: Sub<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_sub<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_sub(a, b).to_array()
}
