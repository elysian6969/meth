use crate::vec::{Element, Lanes, SupportedLanes};
use core::intrinsics::const_eval_select;
use core::ops::{Add, Div, Mul, Rem, Sub};

/// SIMD vector.
#[derive(Debug)]
#[repr(simd)]
pub struct Simd<T, const LANES: usize>(pub [T; LANES]);

impl<T, const LANES: usize> Simd<T, LANES> {
    /// Pointer to the first element.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first element.
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    /// Create an uninitialized SIMD vector.
    #[inline]
    pub const fn uninit() -> Simd<T, LANES> {
        Self(crate::mem::uninit_array())
    }

    /// Get item at the specified index.
    ///
    /// # Safety
    ///
    /// Caller must ensure the index is valid.
    #[inline]
    pub const unsafe fn get(&self, index: usize) -> T
    where
        T: Sized,
    {
        self.as_ptr().add(index).read()
    }

    /// Set the item at the specified index.
    ///
    /// # Safety
    ///
    /// Caller must ensure the index is valid.
    #[inline]
    pub const unsafe fn set(&mut self, index: usize, value: T)
    where
        T: Sized,
    {
        self.as_mut_ptr().add(index).write(value)
    }
}

impl<T, const LANES: usize> const Add<Simd<T, LANES>> for Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    type Output = Simd<T, LANES>;

    #[inline]
    fn add(self, other: Simd<T, LANES>) -> Simd<T, LANES> {
        simd_add(self, other)
    }
}

impl<T, const LANES: usize> const Div<Simd<T, LANES>> for Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    type Output = Simd<T, LANES>;

    #[inline]
    fn div(self, other: Simd<T, LANES>) -> Simd<T, LANES> {
        simd_div(self, other)
    }
}

impl<T, const LANES: usize> const Mul<Simd<T, LANES>> for Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    type Output = Simd<T, LANES>;

    #[inline]
    fn mul(self, other: Simd<T, LANES>) -> Simd<T, LANES> {
        simd_mul(self, other)
    }
}

impl<T, const LANES: usize> const Rem<Simd<T, LANES>> for Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    type Output = Simd<T, LANES>;

    #[inline]
    fn rem(self, other: Simd<T, LANES>) -> Simd<T, LANES> {
        simd_rem(self, other)
    }
}

impl<T, const LANES: usize> const Sub<Simd<T, LANES>> for Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    type Output = Simd<T, LANES>;

    #[inline]
    fn sub(self, other: Simd<T, LANES>) -> Simd<T, LANES> {
        simd_sub(self, other)
    }
}

/// Add two SIMD vectors.
#[inline]
pub const fn simd_add<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD add, scalar, compile-time.
    #[inline]
    pub const fn scalar_add<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Add<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) + b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD add, intrinsic, runtime.
    #[inline]
    pub fn simd_add<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Add<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_add<T>(a: T, b: T) -> T;
        }

        unsafe { simd_add(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_add, simd_add) }
}

/// Divide two SIMD vectors.
#[inline]
pub const fn simd_div<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD divide, scalar, compile-time.
    #[inline]
    pub const fn scalar_div<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Div<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) / b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD divide, intrinsic, runtime.
    #[inline]
    pub fn simd_div<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Div<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_div<T>(a: T, b: T) -> T;
        }

        unsafe { simd_div(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_div, simd_div) }
}

/// Multiply two SIMD vectors.
#[inline]
pub const fn simd_mul<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD multiply, scalar, compile-time.
    #[inline]
    pub const fn scalar_mul<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Mul<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) * b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD multiply, intrinsic, runtime.
    #[inline]
    pub fn simd_mul<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Mul<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_mul<T>(a: T, b: T) -> T;
        }

        unsafe { simd_mul(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_mul, simd_mul) }
}

/// Remainder of two SIMD vectors.
#[inline]
pub const fn simd_rem<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD remainder, scalar, compile-time.
    #[inline]
    pub const fn scalar_rem<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Rem<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) % b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD remainder, intrinsic, runtime.
    #[inline]
    pub fn simd_rem<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Rem<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_rem<T>(a: T, b: T) -> T;
        }

        unsafe { simd_rem(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_rem, simd_rem) }
}

/// Subtract two SIMD vectors.
#[inline]
pub const fn simd_sub<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<LANES>: ~const SupportedLanes,
{
    /// SIMD subtract, scalar, compile-time.
    #[inline]
    pub const fn scalar_sub<T, const LANES: usize>(
        a: Simd<T, LANES>,
        b: Simd<T, LANES>,
    ) -> Simd<T, LANES>
    where
        T: ~const Element,
        T: ~const Sub<Output = T>,
        Lanes<LANES>: ~const SupportedLanes,
    {
        let mut result = Simd::<T, LANES>::uninit();
        let mut i = 0;

        while i < LANES {
            unsafe {
                result.set(i, a.get(i) - b.get(i));
            }

            i += 1;
        }

        result
    }

    /// SIMD subtract, intrinsic, runtime.
    #[inline]
    pub fn simd_sub<T, const LANES: usize>(a: Simd<T, LANES>, b: Simd<T, LANES>) -> Simd<T, LANES>
    where
        T: Element,
        T: Sub<Output = T>,
        Lanes<LANES>: SupportedLanes,
    {
        extern "platform-intrinsic" {
            pub fn simd_sub<T>(a: T, b: T) -> T;
        }

        unsafe { simd_sub(a, b) }
    }

    unsafe { const_eval_select((a, b), scalar_sub, simd_sub) }
}
