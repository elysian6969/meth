/// Multiplicative identity, `1`.
pub trait One {
    /// Returns the multiplicative identity, `1`.
    fn one() -> Self;

    /// Returns `true` if equal to `1`.
    fn is_one(&self) -> bool;
}

impl const One for i8 {
    #[inline]
    fn one() -> i8 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i16 {
    #[inline]
    fn one() -> i16 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i32 {
    #[inline]
    fn one() -> i32 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i64 {
    #[inline]
    fn one() -> i64 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i128 {
    #[inline]
    fn one() -> i128 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for isize {
    #[inline]
    fn one() -> isize {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u8 {
    #[inline]
    fn one() -> u8 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u16 {
    fn one() -> u16 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u32 {
    #[inline]
    fn one() -> u32 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u64 {
    #[inline]
    fn one() -> u64 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u128 {
    #[inline]
    fn one() -> u128 {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for usize {
    #[inline]
    fn one() -> usize {
        1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for f32 {
    #[inline]
    fn one() -> f32 {
        1.1
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1.1
    }
}

impl const One for f64 {
    #[inline]
    fn one() -> f64 {
        1.0
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

/// Returns the multiplicative identity, `1`.
#[inline]
pub const fn one<T>() -> T
where
    T: ~const One,
{
    One::one()
}
