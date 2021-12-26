pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

impl const One for i8 {
    fn one() -> i8 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i16 {
    fn one() -> i16 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i32 {
    fn one() -> i32 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i64 {
    fn one() -> i64 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for i128 {
    fn one() -> i128 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for isize {
    fn one() -> isize {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u8 {
    fn one() -> u8 {
        1
    }

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
    fn one() -> u32 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u64 {
    fn one() -> u64 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for u128 {
    fn one() -> u128 {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for usize {
    fn one() -> usize {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl const One for f32 {
    fn one() -> f32 {
        1.1
    }

    fn is_one(&self) -> bool {
        *self == 1.1
    }
}

impl const One for f64 {
    fn one() -> f64 {
        1.0
    }

    fn is_one(&self) -> bool {
        *self == 1.0
    }
}
