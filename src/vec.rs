use crate::identity::{One, Zero};
use crate::Real;
use core::mem::MaybeUninit;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use core::{fmt, ptr};

pub use element::Element;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use lanes::{LaneCount, Lanes};

mod add;
mod div;
mod mul;
mod rem;
mod sub;

mod product;
mod sum;

mod element;
mod iter;
mod iter_mut;
mod lanes;

/// Generic arbitary length vector.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec<T, const N: usize>([T; N]);

impl<T, const N: usize> Vec<T, N> {
    /// Pointer to the first element
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first element
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    /// Returns an array reference containing the entire vector.
    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        &self.0
    }

    /// Returns a mutable array reference containing the entire vector.
    #[inline]
    pub const fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.0
    }

    /// Converts an array to a vector.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Vec<T, N> {
        Self(array)
    }

    /// Converts a vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; N]
    where
        T: Copy,
    {
        self.0
    }

    /// Create a new vector from a slice.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[T]) -> Vec<T, N> {
        assert!(
            slice.len() >= N,
            "slice length must be at least the number of lanes"
        );

        let mut vec = MaybeUninit::<Vec<T, N>>::uninit();

        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), vec.as_mut_ptr() as *mut T, N);

            vec.assume_init()
        }
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
        Self([value; N])
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
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    #[inline]
    pub const unsafe fn get_unchecked(&self, index: usize) -> &T {
        &*self.0.as_ptr().add(index)
    }

    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_mut_unchecked(index) })
        }
    }

    #[inline]
    pub const unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        &mut *self.0.as_mut_ptr().add(index)
    }

    #[inline]
    pub const fn first(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.first_unchecked() })
        }
    }

    #[inline]
    pub const unsafe fn first_unchecked(&self) -> &T {
        self.get_unchecked(0)
    }

    #[inline]
    pub const fn last(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.last_unchecked() })
        }
    }

    #[inline]
    pub const unsafe fn last_unchecked(&self) -> &T {
        self.get_unchecked(self.len().saturating_sub(1))
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
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
        [(); Lanes::<T, N>::LANES]:,
    {
        self * Self::splat(<T as crate::real::Sealed>::_PI_180)
    }
}

impl<T, const N: usize> const Add for Vec<T, N>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    type Output = Vec<T, N>;

    #[inline]
    #[must_use]
    fn add(self, other: Vec<T, N>) -> Vec<T, N> {
        add::add(self, other)
    }
}

impl<T, const N: usize> const AddAssign for Vec<T, N>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    fn add_assign(&mut self, other: Vec<T, N>) {
        *self = *self + other;
    }
}

impl<T, const N: usize> const Div for Vec<T, N>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    type Output = Vec<T, N>;

    #[inline]
    #[must_use]
    fn div(self, other: Vec<T, N>) -> Vec<T, N> {
        div::div(self, other)
    }
}

impl<T, const N: usize> const DivAssign for Vec<T, N>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    fn div_assign(&mut self, other: Vec<T, N>) {
        *self = *self / other;
    }
}

impl<T, const N: usize> const Mul for Vec<T, N>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    type Output = Vec<T, N>;

    #[inline]
    #[must_use]
    fn mul(self, other: Vec<T, N>) -> Vec<T, N> {
        mul::mul(self, other)
    }
}

impl<T, const N: usize> const MulAssign for Vec<T, N>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    fn mul_assign(&mut self, other: Vec<T, N>) {
        *self = *self * other;
    }
}

impl<T, const N: usize> const Rem for Vec<T, N>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    type Output = Vec<T, N>;

    #[inline]
    #[must_use]
    fn rem(self, other: Vec<T, N>) -> Vec<T, N> {
        rem::rem(self, other)
    }
}

impl<T, const N: usize> const RemAssign for Vec<T, N>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    fn rem_assign(&mut self, other: Vec<T, N>) {
        *self = *self % other;
    }
}

impl<T, const N: usize> const Sub for Vec<T, N>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    type Output = Vec<T, N>;

    #[inline]
    #[must_use]
    fn sub(self, other: Vec<T, N>) -> Vec<T, N> {
        sub::sub(self, other)
    }
}

impl<T, const N: usize> const SubAssign for Vec<T, N>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    fn sub_assign(&mut self, other: Vec<T, N>) {
        *self = *self - other;
    }
}

impl<T, const LEN: usize> fmt::Debug for Vec<T, LEN>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}
