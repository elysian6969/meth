use super::Simd;
use crate::identity::One;
use core::ops::Mul;

#[inline]
#[must_use]
pub unsafe fn simd_product<T, const N: usize>(a: [T; N]) -> T
where
    T: Copy,
    T: One,
    T: Mul<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_reduce_mul_ordered<T, U>(a: T, b: U) -> U;
    }

    let a = Simd::from_array(a);
    let b = <T as One>::one();

    simd_reduce_mul_ordered(a, b)
}
