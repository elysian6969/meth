use crate::identity::{One, Zero};
use crate::Vec;

pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>(Vec<T, { ROWS * COLUMNS }>)
where
    [(); ROWS * COLUMNS]:;

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// Numbers in rows in this matrix.
    pub const ROWS: usize = ROWS;

    /// Numbers in columns in this matrix.
    pub const COLUMNS: usize = COLUMNS;

    /// Get the numbers in rows in this matrix.
    #[inline]
    pub const fn rows(&self) -> usize {
        ROWS
    }

    /// Get the numbers in columns in this matrix.
    #[inline]
    pub const fn columns(&self) -> usize {
        COLUMNS
    }

    /// Creates a new vector with all elements set to the given value.
    #[inline]
    pub const fn splat(value: T) -> Self
    where
        T: ~const Copy,
    {
        Self(Vec::splat(value))
    }

    /// Creates a new vector with all elements set to zero.
    #[inline]
    pub const fn zero() -> Self
    where
        T: ~const Copy,
        T: ~const Zero,
    {
        Self(Vec::zero())
    }

    /// Creates a new vector with all elements set to one.
    #[inline]
    pub const fn one() -> Self
    where
        T: ~const Copy,
        T: ~const One,
    {
        Self(Vec::one())
    }
}
