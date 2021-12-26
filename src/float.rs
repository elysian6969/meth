pub trait Float: Sized {
    const DIGITS: u32;
    const EPSILON: Self;
    const INFINITY: Self;
    const MANTISSA_DIGITS: u32;
    const MAX: Self;
    const MAX_10_EXP: i32;
    const MAX_EXP: i32;
    const MIN: Self;
    const MIN_10_EXP: i32;
    const MIN_EXP: i32;
    const NAN: Self;
    const NEG_INFINITY: Self;

    fn abs(self) -> Self;
    fn asin(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn copysign(self, sign: Self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
}

impl const Float for f32 {
    const DIGITS: u32 = core::f32::DIGITS;
    const EPSILON: Self = core::f32::EPSILON;
    const INFINITY: Self = core::f32::INFINITY;
    const MANTISSA_DIGITS: u32 = core::f32::MANTISSA_DIGITS;
    const MAX: Self = core::f32::MAX;
    const MAX_10_EXP: i32 = core::f32::MAX_10_EXP;
    const MAX_EXP: i32 = core::f32::MAX_EXP;
    const MIN: Self = core::f32::MIN;
    const MIN_10_EXP: i32 = core::f32::MIN_10_EXP;
    const MIN_EXP: i32 = core::f32::MIN_EXP;
    const NAN: Self = core::f32::NAN;
    const NEG_INFINITY: Self = core::f32::NEG_INFINITY;

    #[inline]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asinf(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        libm::copysignf(self, sign)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cosf(self)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sinf(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincosf(self)
    }
}

impl const Float for f64 {
    const DIGITS: u32 = f64::DIGITS;
    const EPSILON: Self = f64::EPSILON;
    const INFINITY: Self = f64::INFINITY;
    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    const MAX: Self = f64::MAX;
    const MAX_10_EXP: i32 = f64::MAX_10_EXP;
    const MAX_EXP: i32 = f64::MAX_EXP;
    const MIN: Self = f64::MIN;
    const MIN_10_EXP: i32 = f64::MIN_10_EXP;
    const MIN_EXP: i32 = f64::MIN_EXP;
    const NAN: Self = f64::NAN;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    #[inline]
    fn abs(self) -> Self {
        libm::fabs(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asin(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        libm::copysign(self, sign)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cos(self)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sin(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincos(self)
    }
}
