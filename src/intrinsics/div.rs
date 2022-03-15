use super::Simd;
use core::ops::Div;

#[inline]
#[must_use]
pub unsafe fn simd_div<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N]
where
    T: Copy,
    T: Div<Output = T>,
{
    extern "platform-intrinsic" {
        fn simd_div<T>(a: T, b: T) -> T;
    }

    let a = Simd::from_array(a);
    let b = Simd::from_array(b);

    simd_div(a, b).to_array()
}
