/// Additive identity, `0`.
pub trait Zero {
    /// Returns the additive identity, `0`.
    fn zero() -> Self;

    /// Returns `true` if equal to `0`.
    fn is_zero(&self) -> bool;
}

impl const Zero for i8 {
    #[inline]
    fn zero() -> i8 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i16 {
    #[inline]
    fn zero() -> i16 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i32 {
    #[inline]
    fn zero() -> i32 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i64 {
    #[inline]
    fn zero() -> i64 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i128 {
    #[inline]
    fn zero() -> i128 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for isize {
    #[inline]
    fn zero() -> isize {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u8 {
    #[inline]
    fn zero() -> u8 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u16 {
    #[inline]
    fn zero() -> u16 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u32 {
    #[inline]
    fn zero() -> u32 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u64 {
    #[inline]
    fn zero() -> u64 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u128 {
    #[inline]
    fn zero() -> u128 {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for usize {
    #[inline]
    fn zero() -> usize {
        0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for f32 {
    #[inline]
    fn zero() -> f32 {
        0.0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl const Zero for f64 {
    #[inline]
    fn zero() -> f64 {
        0.0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

/// Returns the additive identity, `0`.
#[inline]
pub const fn zero<T>() -> T
where
    T: ~const Zero,
{
    Zero::zero()
}
