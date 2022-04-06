use crate::identity::{One, Zero};
use crate::Real;
use core::fmt;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Deref, DerefMut};

pub use element::Element;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use lanes::{LaneCount, Lanes};

mod product;
mod sum;

mod element;
mod iter;
mod iter_mut;
mod lanes;

/// Generic arbitary length vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec<T, const N: usize> {
    array: [T; N],
}

impl<T, const N: usize> Vec<T, N> {
    /// Converts an array to a vector.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Vec<T, N> {
        Self { array }
    }

    /// Create a new vector from a slice.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[T]) -> Vec<T, N>
    where
        T: Copy,
    {
        let slice = &slice[..N];

        // SAFETY: above expression ensures the slice length is sufficient to copy [T; N]
        unsafe { slice.as_ptr().cast::<Vec<T, N>>().read() }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Creates a new vector with all elements set to the given value.
    #[inline]
    #[must_use]
    pub const fn splat(value: T) -> Vec<T, N>
    where
        T: Copy,
    {
        Self::from_array([value; N])
    }

    /// Creates a new vector with all elements set to zero.
    #[inline]
    #[must_use]
    pub const fn zero() -> Vec<T, N>
    where
        T: Copy,
        T: ~const Zero,
    {
        Self::splat(Zero::zero())
    }

    /// Creates a new vector with all elements set to one.
    #[inline]
    #[must_use]
    pub const fn one() -> Vec<T, N>
    where
        T: Copy,
        T: ~const One,
    {
        Self::splat(One::one())
    }

    #[inline]
    pub const fn iter(&self) -> Iter<'_, T, N> {
        Iter::new(self)
    }

    #[inline]
    pub const fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut::new(self)
    }

    #[inline]
    pub const fn distance(self, other: Vec<T, N>) -> T
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        T: ~const Sub<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        <T as Real>::sqrt(self.distance_squared(other))
    }

    #[inline]
    pub const fn distance_squared(self, other: Vec<T, N>) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        T: ~const Sub<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        (self - other).magnitude_squared()
    }

    #[inline]
    pub const fn dot(self, other: Vec<T, N>) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        (self * other).sum()
    }

    #[inline]
    pub const fn magnitude(self) -> T
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        <T as Real>::sqrt(self.magnitude_squared())
    }

    #[inline]
    pub const fn magnitude_squared(self) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        self.dot(self)
    }

    #[inline]
    pub const fn product(self) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        product::product(self)
    }

    #[inline]
    pub const fn sum(self) -> T
    where
        T: ~const Element,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        sum::sum(self)
    }

    #[inline]
    #[must_use]
    pub const fn to_degrees(self) -> Vec<T, N>
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        self * Self::splat(<T as crate::real::Sealed>::_180_PI)
    }

    #[inline]
    #[must_use]
    pub const fn to_radians(self) -> Vec<T, N>
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); <Lanes<T, N> as LaneCount>::LANES]:,
    {
        self * Self::splat(<T as crate::real::Sealed>::_PI_180)
    }
}

impl<T, const N: usize> const Deref for Vec<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &[T; N] {
        &self.array
    }
}

impl<T, const N: usize> const DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut [T; N] {
        &mut self.array
    }
}

macro_rules! impl_op {
    { $trait:ident, $trait_assign:ident, $fn:ident, $fn_assign:ident, $fn_scalar:ident, $fn_simd:ident, $op:tt } => {
        impl<T, const N: usize> const $trait for Vec<T, N>
        where
            T: ~const Element,
            T: ~const $trait<Output = T>,
            Lanes<T, N>: LaneCount,
            [(); <Lanes<T, N> as LaneCount>::LANES]:,
        {
            type Output = Vec<T, N>;

            #[inline]
            #[must_use]
            fn $fn(self, other: Vec<T, N>) -> Vec<T, N> {
                // called in const contexts
                #[inline]
                #[must_use]
                const fn $fn_scalar<T, const N: usize>(mut a: Vec<T, N>, b: Vec<T, N>) -> Vec<T, N>
                where
                    T: ~const Element,
                    T: ~const $trait<Output = T>,
                    Lanes<T, N>: LaneCount,
                    [(); Lanes::<T, N>::LANES]:,
                {
                    let mut a_iter = a.iter_mut();
                    let mut b_iter = b.iter();

                    while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
                        *a = *a $op *b;
                    }

                    a
                }

                // called in non-const contexts
                #[inline]
                #[must_use]
                fn $fn_simd<T, const N: usize>(a: Vec<T, N>, b: Vec<T, N>) -> Vec<T, N>
                where
                    T: Element,
                    T: $trait<Output = T>,
                    Lanes<T, N>: LaneCount,
                    [(); Lanes::<T, N>::LANES]:,
                {
                    unsafe {
                        let mut a_iter = a.array.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);
                        let mut b_iter = b.array.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);

                        while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
                            let a = a.as_ptr() as *mut [T; <Lanes<T, N> as LaneCount>::LANES];
                            let b = b.as_ptr() as *mut [T; <Lanes<T, N> as LaneCount>::LANES];

                            *a = crate::intrinsics::$fn_simd(*a, *b);
                        }

                        let mut a_iter = a_iter.remainder().iter();
                        let mut b_iter = b_iter.remainder().iter();

                        while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
                            let a = &mut *(a as *const T as *mut T);

                            *a = *a $op *b;
                        }

                        a
                    }
                }

                unsafe { core::intrinsics::const_eval_select((self, other), $fn_scalar, $fn_simd) }
            }
        }

        impl<T, const N: usize> const $trait_assign for Vec<T, N>
        where
            T: ~const Element,
            T: ~const $trait <Output = T>,
            Lanes<T, N>: LaneCount,
            [(); <Lanes<T, N> as LaneCount>::LANES]:,
        {
            #[inline]
            #[must_use]
            fn $fn_assign(&mut self, other: Vec<T, N>) {
                *self = *self $op other;
            }
        }
    }
}

impl_op! { Add, AddAssign, add, add_assign, scalar_add, simd_add, + }
impl_op! { Div, DivAssign, div, div_assign, scalar_div, simd_div, / }
impl_op! { Mul, MulAssign, mul, mul_assign, scalar_mul, simd_mul, * }
impl_op! { Rem, RemAssign, rem, rem_assign, scalar_rem, simd_rem, % }
impl_op! { Sub, SubAssign, sub, sub_assign, scalar_sub, simd_sub, - }

impl<T, const LEN: usize> fmt::Debug for Vec<T, LEN>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.array, fmt)
    }
}
