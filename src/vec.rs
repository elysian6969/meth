use crate::ops::Simd;
use crate::{One, Zero};
use core::{fmt, ptr};

pub use self::element::Element;
pub use self::lanes::{Lanes, SupportedLanes};

mod element;
mod lanes;

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

impl<T, const LEN: usize> Vec<T, LEN> {
    /// Length of this vector.
    pub const LEN: usize = LEN;

    /// Number of lanes for SIMD operations.
    pub const LANES: usize = {
        let lanes = LEN.next_power_of_two() >> 1;

        if lanes > 8 {
            8
        } else {
            lanes
        }
    };

    /// Number of SIMD vectors needes for this vector.
    pub const VECTORS: usize = LEN / Self::LANES;

    /// Remainder of elements that cannot fit into SIMD vectors.
    pub const REMAINDER: usize = LEN % (Self::VECTORS * Self::LANES);

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

    /// Create a `Vec` from `slice`.
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
        T: ~const Copy + ~const Zero,
    {
        Self::splat(Zero::zero())
    }

    /// Creates a new vector with all elements set to one.
    #[inline]
    pub const fn one() -> Vec<T, LEN>
    where
        T: ~const Copy + ~const One,
    {
        Self::splat(One::one())
    }

    /// Create a `Vec` from uninitialized bytes.
    #[inline]
    pub const fn uninit() -> Vec<T, LEN> {
        Self(crate::mem::uninit_array())
    }

    #[inline]
    pub const fn to_simd(
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
}

impl<T, const LEN: usize> fmt::Debug for Vec<T, LEN>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}
