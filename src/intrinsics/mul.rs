use super::Simd;
use core::ops::Mul;

#[inline]
#[must_use]
pub unsafe fn simd_mul<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: Mul<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_mul<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_mul(a, b).to_array()
}
