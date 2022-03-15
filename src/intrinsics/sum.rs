use super::Simd;
use crate::identity::Zero;
use core::ops::Add;

#[inline]
#[must_use]
pub unsafe fn simd_sum<T, const N: usize>(a: [T; N]) -> T
where
    T: Copy,
    T: Zero,
    T: Add<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_reduce_add_ordered<T, U>(a: T, b: U) -> U;
    }

    let a = Simd::from_array(a);
    let b = <T as Zero>::zero();

    simd_reduce_add_ordered(a, b)
}
