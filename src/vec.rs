use crate::identity::{One, Zero};
use crate::ops::Simd;
use crate::Real;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use core::{fmt, ptr};

pub use self::element::Element;
pub use self::lanes::{Lanes, SupportedLanes};

mod element;
mod lanes;

/// Generic arbitary length vector.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec<T, const LEN: usize>([T; LEN]);

/// Helper type for splitting into SIMD vectors and remainder.
struct Split<T, const LANES: usize, const VECTORS: usize, const REMAINDER: usize> {
    pub vectors: [Simd<T, LANES>; VECTORS],
    pub remainder: [T; REMAINDER],
}

impl<T, const LANES: usize, const VECTORS: usize, const REMAINDER: usize>
    Split<T, LANES, VECTORS, REMAINDER>
{
    #[inline]
    pub const fn new(ptr: *const T) -> Self {
        let mut vectors = crate::mem::uninit_array();
        let mut remainder = crate::mem::uninit_array();

        unsafe {
            ptr::copy_nonoverlapping(ptr, vectors.as_mut_ptr() as *mut T, VECTORS * LANES);
            ptr::copy_nonoverlapping(ptr.add(VECTORS * LANES), remainder.as_mut_ptr(), REMAINDER);
        }

        Self { vectors, remainder }
    }
}

#[inline]
const fn deduce_lanes(len: usize) -> usize {
    if len.is_power_of_two() {
        return len;
    }

    let lanes = len.next_power_of_two() >> 1;

    if lanes > 8 {
        8
    } else {
        lanes
    }
}

impl<T, const LEN: usize> Vec<T, LEN> {
    /// Length of this vector.
    pub const LEN: usize = LEN;

    /// Number of lanes for SIMD operations.
    pub const LANES: usize = deduce_lanes(LEN);

    /// Number of SIMD vectors needed for this vector.
    pub const VECTORS: usize = LEN / Self::LANES;

    /// Remainder of elements that cannot fit into SIMD vectors.
    pub const REMAINDER: usize = LEN % (Self::VECTORS * Self::LANES);

    /// Returns the length of this vector.
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Returns the number of lanes for SIMD operations.
    #[inline]
    pub const fn lanes(&self) -> usize {
        Self::LANES
    }

    /// Returns the number of SIMD vectors needed for this vector.
    #[inline]
    pub const fn vectors(&self) -> usize {
        Self::VECTORS
    }

    /// Returns the remainder of elements that cannot fit into SIMD vectors.
    #[inline]
    pub const fn remainder(&self) -> usize {
        Self::REMAINDER
    }

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

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn as_vector_ptr(&self) -> *const Simd<T, { Self::LANES }> {
        self.as_ptr() as *const Simd<T, { Self::LANES }>
    }

    #[inline]
    pub(crate) const fn as_vector_mut_ptr(&mut self) -> *mut Simd<T, { Self::LANES }> {
        self.as_mut_ptr() as *mut Simd<T, { Self::LANES }>
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn as_remainder_ptr(&self) -> *const T
    where
        [(); Self::LANES]:,
    {
        // SAFETY: It's a valid offset.
        unsafe { self.as_vector_ptr().add(Self::VECTORS) as *const T }
    }

    #[inline]
    pub(crate) const fn as_remainder_mut_ptr(&mut self) -> *mut T
    where
        [(); Self::LANES]:,
    {
        // SAFETY: It's a valid offset.
        unsafe { self.as_vector_mut_ptr().add(Self::VECTORS) as *mut T }
    }

    /// Returns an array reference containing the entire vector.
    #[inline]
    pub const fn as_array(&self) -> &[T; LEN] {
        &self.0
    }

    /// Returns a mutable array reference containing the entire vector.
    #[inline]
    pub const fn as_mut_array(&mut self) -> &mut [T; LEN] {
        &mut self.0
    }

    /// Converts an array to a vector.
    #[inline]
    pub const fn from_array(array: [T; LEN]) -> Self {
        Self(array)
    }

    /// Converts a vector to an array.
    #[inline]
    pub const fn to_array(self) -> [T; LEN]
    where
        T: ~const Copy,
    {
        self.0
    }

    /// Create a new vector from a slice.
    #[inline]
    pub const fn from_slice(slice: &[T]) -> Self {
        assert!(
            slice.len() >= LEN,
            "slice length must be at least the number of lanes"
        );

        let mut vec = Vec::uninit();

        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), vec.as_mut_ptr(), LEN);
        }

        vec
    }

    /// Creates a new vector with all elements set to the given value.
    #[inline]
    pub const fn splat(value: T) -> Vec<T, LEN>
    where
        T: ~const Copy,
    {
        Self([value; LEN])
    }

    /// Creates a new vector with all elements set to zero.
    #[inline]
    pub const fn zero() -> Vec<T, LEN>
    where
        T: ~const Copy,
        T: ~const Zero,
    {
        Self::splat(Zero::zero())
    }

    /// Creates a new vector with all elements set to one.
    #[inline]
    pub const fn one() -> Vec<T, LEN>
    where
        T: ~const Copy,
        T: ~const One,
    {
        Self::splat(One::one())
    }

    /// Create a new vector from uninitialized bytes.
    #[inline]
    pub const fn uninit() -> Vec<T, LEN> {
        Self(crate::mem::uninit_array())
    }

    #[inline]
    pub(crate) const fn to_simd(
        self,
    ) -> (
        [Simd<T, { Self::LANES }>; Self::VECTORS],
        [T; Self::REMAINDER],
    )
    where
        T: ~const Copy,
        T: ~const Element,
    {
        let Split { vectors, remainder } =
            Split::<T, { Self::LANES }, { Self::VECTORS }, { Self::REMAINDER }>::new(self.as_ptr());

        (vectors, remainder)
    }

    #[inline]
    pub const fn to_degrees(self) -> Self
    where
        T: ~const Copy,
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
        [(); Vec::<T, LEN>::LANES]:,
        [(); Vec::<T, LEN>::VECTORS]:,
        [(); Vec::<T, LEN>::REMAINDER]:,
    {
        self * <T as crate::real::Sealed>::_180_PI
    }

    #[inline]
    pub const fn to_radians(self) -> Self
    where
        T: ~const Copy,
        T: ~const Element,
        T: ~const Real,
        T: ~const Mul<Output = T>,
        Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
        [(); Vec::<T, LEN>::LANES]:,
        [(); Vec::<T, LEN>::VECTORS]:,
        [(); Vec::<T, LEN>::REMAINDER]:,
    {
        self * <T as crate::real::Sealed>::_PI_180
    }
}

// Vec<T, LEN> and Vec<T, LEN> operations.
impl<T, const LEN: usize> const Add<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn add(self, other: Vec<T, LEN>) -> Vec<T, LEN> {
        vec_add(self, other)
    }
}

impl<T, const LEN: usize> const AddAssign<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn add_assign(&mut self, other: Vec<T, LEN>) {
        *self = *self + other;
    }
}

impl<T, const LEN: usize> const Div<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn div(self, other: Vec<T, LEN>) -> Vec<T, LEN> {
        vec_div(self, other)
    }
}

impl<T, const LEN: usize> const DivAssign<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn div_assign(&mut self, other: Vec<T, LEN>) {
        *self = *self / other;
    }
}

impl<T, const LEN: usize> const Mul<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn mul(self, other: Vec<T, LEN>) -> Vec<T, LEN> {
        vec_mul(self, other)
    }
}

impl<T, const LEN: usize> const MulAssign<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn mul_assign(&mut self, other: Vec<T, LEN>) {
        *self = *self * other;
    }
}

impl<T, const LEN: usize> const Rem<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn rem(self, other: Vec<T, LEN>) -> Vec<T, LEN> {
        vec_rem(self, other)
    }
}

impl<T, const LEN: usize> const RemAssign<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn rem_assign(&mut self, other: Vec<T, LEN>) {
        *self = *self % other;
    }
}

impl<T, const LEN: usize> const Sub<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn sub(self, other: Vec<T, LEN>) -> Vec<T, LEN> {
        vec_sub(self, other)
    }
}

impl<T, const LEN: usize> const SubAssign<Vec<T, LEN>> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn sub_assign(&mut self, other: Vec<T, LEN>) {
        *self = *self - other;
    }
}

// Vec<T, LEN> and T operations.
impl<T, const LEN: usize> const Add<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn add(self, element: T) -> Vec<T, LEN> {
        self + Self::splat(element)
    }
}

impl<T, const LEN: usize> const AddAssign<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn add_assign(&mut self, element: T) {
        *self = *self + element;
    }
}

impl<T, const LEN: usize> const Div<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn div(self, element: T) -> Vec<T, LEN> {
        self / Self::splat(element)
    }
}

impl<T, const LEN: usize> const DivAssign<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn div_assign(&mut self, element: T) {
        *self = *self / element;
    }
}

impl<T, const LEN: usize> const Mul<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn mul(self, element: T) -> Vec<T, LEN> {
        self * Self::splat(element)
    }
}

impl<T, const LEN: usize> const MulAssign<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn mul_assign(&mut self, element: T) {
        *self = *self * element;
    }
}

impl<T, const LEN: usize> const Rem<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn rem(self, element: T) -> Vec<T, LEN> {
        self % Self::splat(element)
    }
}

impl<T, const LEN: usize> const RemAssign<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn rem_assign(&mut self, element: T) {
        *self = *self % element;
    }
}

impl<T, const LEN: usize> const Sub<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    type Output = Vec<T, LEN>;

    #[inline]
    fn sub(self, element: T) -> Vec<T, LEN> {
        self - Self::splat(element)
    }
}

impl<T, const LEN: usize> const SubAssign<T> for Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    #[inline]
    fn sub_assign(&mut self, element: T) {
        *self = *self - element;
    }
}

/// Add two vectors.
#[inline]
pub const fn vec_add<T, const LEN: usize>(a: Vec<T, LEN>, b: Vec<T, LEN>) -> Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    let (a_vectors, a_remainder) = a.to_simd();
    let (b_vectors, b_remainder) = b.to_simd();
    let mut output = Vec::<T, LEN>::uninit();
    let mut i = 0;

    unsafe {
        while i < { Vec::<T, LEN>::VECTORS } {
            output
                .as_vector_mut_ptr()
                .add(i)
                .write(a_vectors.as_ptr().add(i).read() + b_vectors.as_ptr().add(i).read());

            i += 1;
        }

        i = 0;

        while i < { Vec::<T, LEN>::REMAINDER } {
            output
                .as_remainder_mut_ptr()
                .add(i)
                .write(a_remainder.as_ptr().add(i).read() + b_remainder.as_ptr().add(i).read());

            i += 1;
        }
    }

    output
}

/// Divide two vectors.
#[inline]
pub const fn vec_div<T, const LEN: usize>(a: Vec<T, LEN>, b: Vec<T, LEN>) -> Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Div<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    let (a_vectors, a_remainder) = a.to_simd();
    let (b_vectors, b_remainder) = b.to_simd();
    let mut output = Vec::<T, LEN>::uninit();
    let mut i = 0;

    unsafe {
        while i < { Vec::<T, LEN>::VECTORS } {
            output
                .as_vector_mut_ptr()
                .add(i)
                .write(a_vectors.as_ptr().add(i).read() / b_vectors.as_ptr().add(i).read());

            i += 1;
        }

        i = 0;

        while i < { Vec::<T, LEN>::REMAINDER } {
            output
                .as_remainder_mut_ptr()
                .add(i)
                .write(a_remainder.as_ptr().add(i).read() / b_remainder.as_ptr().add(i).read());

            i += 1;
        }
    }

    output
}

/// Multiply two vectors.
#[inline]
pub const fn vec_mul<T, const LEN: usize>(a: Vec<T, LEN>, b: Vec<T, LEN>) -> Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Mul<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    let (a_vectors, a_remainder) = a.to_simd();
    let (b_vectors, b_remainder) = b.to_simd();
    let mut output = Vec::<T, LEN>::uninit();
    let mut i = 0;

    unsafe {
        while i < { Vec::<T, LEN>::VECTORS } {
            output
                .as_vector_mut_ptr()
                .add(i)
                .write(a_vectors.as_ptr().add(i).read() * b_vectors.as_ptr().add(i).read());

            i += 1;
        }

        i = 0;

        while i < { Vec::<T, LEN>::REMAINDER } {
            output
                .as_remainder_mut_ptr()
                .add(i)
                .write(a_remainder.as_ptr().add(i).read() * b_remainder.as_ptr().add(i).read());

            i += 1;
        }
    }

    output
}

/// Remainder two vectors.
#[inline]
pub const fn vec_rem<T, const LEN: usize>(a: Vec<T, LEN>, b: Vec<T, LEN>) -> Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Rem<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    let (a_vectors, a_remainder) = a.to_simd();
    let (b_vectors, b_remainder) = b.to_simd();
    let mut output = Vec::<T, LEN>::uninit();
    let mut i = 0;

    unsafe {
        while i < { Vec::<T, LEN>::VECTORS } {
            output
                .as_vector_mut_ptr()
                .add(i)
                .write(a_vectors.as_ptr().add(i).read() % b_vectors.as_ptr().add(i).read());

            i += 1;
        }

        i = 0;

        while i < { Vec::<T, LEN>::REMAINDER } {
            output
                .as_remainder_mut_ptr()
                .add(i)
                .write(a_remainder.as_ptr().add(i).read() % b_remainder.as_ptr().add(i).read());

            i += 1;
        }
    }

    output
}

/// Subtract two vectors.
#[inline]
pub const fn vec_sub<T, const LEN: usize>(a: Vec<T, LEN>, b: Vec<T, LEN>) -> Vec<T, LEN>
where
    T: ~const Copy,
    T: ~const Element,
    T: ~const Sub<Output = T>,
    Lanes<{ Vec::<T, LEN>::LANES }>: SupportedLanes,
    [(); Vec::<T, LEN>::LANES]:,
    [(); Vec::<T, LEN>::VECTORS]:,
    [(); Vec::<T, LEN>::REMAINDER]:,
{
    let (a_vectors, a_remainder) = a.to_simd();
    let (b_vectors, b_remainder) = b.to_simd();
    let mut output = Vec::<T, LEN>::uninit();
    let mut i = 0;

    unsafe {
        while i < { Vec::<T, LEN>::VECTORS } {
            output
                .as_vector_mut_ptr()
                .add(i)
                .write(a_vectors.as_ptr().add(i).read() - b_vectors.as_ptr().add(i).read());

            i += 1;
        }

        i = 0;

        while i < { Vec::<T, LEN>::REMAINDER } {
            output
                .as_remainder_mut_ptr()
                .add(i)
                .write(a_remainder.as_ptr().add(i).read() - b_remainder.as_ptr().add(i).read());

            i += 1;
        }
    }

    output
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
