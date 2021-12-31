use crate::{One, Zero};

pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>([T; ROWS * COLUMNS])
where
    [(); ROWS * COLUMNS]:;

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    pub const ROWS: usize = ROWS;
    pub const COLUMNS: usize = COLUMNS;

    pub const fn zero() -> Self
    where
        T: ~const Zero + Copy,
    {
        Self([Zero::zero(); ROWS * COLUMNS])
    }

    pub const fn one() -> Self
    where
        T: ~const One + Copy,
    {
        Self([One::one(); ROWS * COLUMNS])
    }
}
