use super::Simd;
use core::ops::Rem;

#[inline]
#[must_use]
pub unsafe fn simd_rem<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: Rem<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_rem<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_rem(a, b).to_array()
}
