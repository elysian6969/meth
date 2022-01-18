use crate::identity::{One, Zero};
use crate::{Element, LaneCount, Lanes, Real, Vec};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use core::ptr;

/// Specialization of `Vec<T, 4>`.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        ptr::addr_of!(self.x)
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        ptr::addr_of_mut!(self.x)
    }

    #[inline]
    pub const fn as_array(&self) -> &[T; 4] {
        unsafe { &*(self.as_ptr() as *const [T; 4]) }
    }

    #[inline]
    pub const fn as_mut_array(&mut self) -> &mut [T; 4] {
        unsafe { &mut *(self.as_mut_ptr() as *mut [T; 4]) }
    }

    /// Create a new `Vec4<T>` from x, and y coordinates, setting y, and w to zero.
    pub const fn from_xy(x: T, y: T) -> Self
    where
        T: ~const Zero,
    {
        Self {
            x,
            y,
            z: <T as Zero>::zero(),
            w: <T as Zero>::zero(),
        }
    }

    /// Create a new `Vec4<T>` from x, y, and z coordinates, setting w to zero.
    pub const fn from_xyz(x: T, y: T, z: T) -> Self
    where
        T: ~const Zero,
    {
        Self {
            x,
            y,
            z,
            w: <T as Zero>::zero(),
        }
    }

    /// Create a new `Vec4<T>` from x, y, z, and w coordinates.
    pub const fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Converts an array to a vector.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Vec4<T>
    where
        T: Copy,
    {
        let [x, y, z, w] = array;

        Self { x, y, z, w }
    }

    /// Converts a vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4]
    where
        T: Copy,
    {
        *self.as_array()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        4
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Creates a new vector with all elements set to the given value.
    #[inline]
    #[must_use]
    pub const fn splat(value: T) -> Vec4<T>
    where
        T: Copy,
    {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    /// Creates a new vector with all elements set to zero.
    #[inline]
    #[must_use]
    pub const fn zero() -> Vec4<T>
    where
        T: Copy,
        T: ~const Zero,
    {
        Self::splat(Zero::zero())
    }

    /// Creates a new vector with all elements set to one.
    #[inline]
    #[must_use]
    pub const fn one() -> Vec4<T>
    where
        T: Copy,
        T: ~const One,
    {
        Self::splat(One::one())
    }

    #[inline]
    pub const fn from_vec(vec: Vec<T, 4>) -> Vec4<T>
    where
        T: Copy,
    {
        unsafe {
            Self {
                x: *vec.get_unchecked(0),
                y: *vec.get_unchecked(1),
                z: *vec.get_unchecked(2),
                w: *vec.get_unchecked(3),
            }
        }
    }

    #[inline]
    pub const fn to_vec(self) -> Vec<T, 4>
    where
        T: Copy,
    {
        Vec::from_array(*self.as_array())
    }

    #[inline]
    pub const fn distance(self, other: Vec4<T>) -> T
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        T: ~const Sub<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().distance(other.to_vec())
    }

    #[inline]
    pub const fn distance_squared(self, other: Vec4<T>) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        T: ~const Sub<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().distance_squared(other.to_vec())
    }

    #[inline]
    pub const fn dot(self, other: Vec4<T>) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().dot(other.to_vec())
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
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().magnitude()
    }

    #[inline]
    pub const fn magnitude_squared(self) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        T: ~const Mul<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().magnitude_squared()
    }

    #[inline]
    pub const fn product(self) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Mul<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().product()
    }

    #[inline]
    pub const fn sum(self) -> T
    where
        T: ~const Element,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        self.to_vec().sum()
    }

    #[inline]
    #[must_use]
    pub const fn to_degrees(self) -> Vec4<T>
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        Self::from_vec(self.to_vec().to_degrees())
    }

    #[inline]
    #[must_use]
    pub const fn to_radians(self) -> Vec4<T>
    where
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<T, 4>: LaneCount,
        [(); <Lanes<T, 4> as LaneCount>::LANES]:,
    {
        Self::from_vec(self.to_vec().to_radians())
    }
}

impl<T> const Add for Vec4<T>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    type Output = Vec4<T>;

    #[inline]
    #[must_use]
    fn add(self, other: Vec4<T>) -> Vec4<T> {
        Self::from_vec(self.to_vec() + other.to_vec())
    }
}

impl<T> const AddAssign for Vec4<T>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    #[inline]
    fn add_assign(&mut self, other: Vec4<T>) {
        *self = *self + other;
    }
}

impl<T> const Div for Vec4<T>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    type Output = Vec4<T>;

    #[inline]
    #[must_use]
    fn div(self, other: Vec4<T>) -> Vec4<T> {
        Self::from_vec(self.to_vec() / other.to_vec())
    }
}

impl<T> const DivAssign for Vec4<T>
where
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    #[inline]
    fn div_assign(&mut self, other: Vec4<T>) {
        *self = *self / other;
    }
}

impl<T> const Mul for Vec4<T>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    type Output = Vec4<T>;

    #[inline]
    #[must_use]
    fn mul(self, other: Vec4<T>) -> Vec4<T> {
        Self::from_vec(self.to_vec() * other.to_vec())
    }
}

impl<T> const MulAssign for Vec4<T>
where
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    #[inline]
    fn mul_assign(&mut self, other: Vec4<T>) {
        *self = *self * other;
    }
}

impl<T> const Rem for Vec4<T>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    type Output = Vec4<T>;

    #[inline]
    #[must_use]
    fn rem(self, other: Vec4<T>) -> Vec4<T> {
        Self::from_vec(self.to_vec() % other.to_vec())
    }
}

impl<T> const RemAssign for Vec4<T>
where
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    #[inline]
    fn rem_assign(&mut self, other: Vec4<T>) {
        *self = *self % other;
    }
}

impl<T> const Sub for Vec4<T>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    type Output = Vec4<T>;

    #[inline]
    #[must_use]
    fn sub(self, other: Vec4<T>) -> Vec4<T> {
        Self::from_vec(self.to_vec() - other.to_vec())
    }
}

impl<T> const SubAssign for Vec4<T>
where
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<T, 4>: LaneCount,
    [(); <Lanes<T, 4> as LaneCount>::LANES]:,
{
    #[inline]
    fn sub_assign(&mut self, other: Vec4<T>) {
        *self = *self - other;
    }
}
