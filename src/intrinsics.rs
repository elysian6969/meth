use crate::identity::{One, Zero};
use core::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug)]
#[repr(simd)]
struct Simd<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Simd<T, N> {
    /// Converts an array to a SIMD vector.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    /// Converts a SIMD vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; N]
    where
        T: Copy,
    {
        self.0
    }
}

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
