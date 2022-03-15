use super::Simd;
use core::ops::Add;

#[inline]
#[must_use]
pub unsafe fn simd_add<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: Add<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_add<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_add(a, b).to_array()
}
