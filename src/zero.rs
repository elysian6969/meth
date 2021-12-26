pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

impl const Zero for i8 {
    fn zero() -> i8 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i16 {
    fn zero() -> i16 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i32 {
    fn zero() -> i32 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i64 {
    fn zero() -> i64 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for i128 {
    fn zero() -> i128 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for isize {
    fn zero() -> isize {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u8 {
    fn zero() -> u8 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u16 {
    fn zero() -> u16 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u32 {
    fn zero() -> u32 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u64 {
    fn zero() -> u64 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for u128 {
    fn zero() -> u128 {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for usize {
    fn zero() -> usize {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl const Zero for f32 {
    fn zero() -> f32 {
        0.0
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl const Zero for f64 {
    fn zero() -> f64 {
        0.0
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}
