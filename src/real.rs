pub(crate) use sealed::Sealed;

mod sealed;

pub trait Real: Sealed {
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

    /// Euler's number (e)
    const E: Self;

    /// 1/π
    const FRAC_1_PI: Self;

    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;

    /// 2/π
    const FRAC_2_PI: Self;

    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;

    /// π/2
    const FRAC_PI_2: Self;

    /// π/3
    const FRAC_PI_3: Self;

    /// π/4
    const FRAC_PI_4: Self;

    /// π/6
    const FRAC_PI_6: Self;

    /// π/8
    const FRAC_PI_8: Self;

    /// ln(2)
    const LN_2: Self;

    /// ln(10)
    const LN_10: Self;

    /// log<sub>2</sub>(10)
    const LOG2_10: Self;

    /// log<sub>2</sub>(e)
    const LOG2_E: Self;

    /// log<sub>10</sub>(2)
    const LOG10_2: Self;

    /// log<sub>10</sub>(e)
    const LOG10_E: Self;

    /// Archimedes' constant (π)
    const PI: Self;

    /// sqrt(2)
    const SQRT_2: Self;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: Self;

    /// Computes the absolute value of `self`. Returns `NAN` if the
    /// number is `NAN`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.5_f32;
    /// let y = -3.5_f32;
    ///
    /// let abs_difference_x = (x.abs() - x).abs();
    /// let abs_difference_y = (y.abs() - (-y)).abs();
    ///
    /// assert!(abs_difference_x <= f32::EPSILON);
    /// assert!(abs_difference_y <= f32::EPSILON);
    ///
    /// assert!(f32::NAN.abs().is_nan());
    /// ```
    #[must_use]
    fn abs(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f32::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn asin(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0f32;
    /// let y1 = -3.0f32;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0f32;
    /// let y2 = 3.0f32;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-<f32 as Real>::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * <f32 as Real>::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= f32::EPSILON);
    /// assert!(abs_difference_2 <= f32::EPSILON);
    /// ```
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// Restrict a value to a certain interval unless it is NaN.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise this returns `self`.
    ///
    /// Note that this function returns NaN if the initial value was NaN as
    /// well.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!((-3.0f32).clamp(-2.0, 1.0) == -2.0);
    /// assert!((0.0f32).clamp(-2.0, 1.0) == 0.0);
    /// assert!((2.0f32).clamp(-2.0, 1.0) == 1.0);
    /// assert!((f32::NAN).clamp(-2.0, 1.0).is_nan());
    /// ```
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise
    /// equal to `-self`. If `self` is a `NAN`, then a `NAN` with the sign of
    /// `sign` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f32;
    ///
    /// assert_eq!(f.copysign(0.42), 3.5_f32);
    /// assert_eq!(f.copysign(-0.42), -3.5_f32);
    /// assert_eq!((-f).copysign(0.42), 3.5_f32);
    /// assert_eq!((-f).copysign(-0.42), -3.5_f32);
    ///
    /// assert!(f32::NAN.copysign(1.0).is_nan());
    /// ```
    #[must_use]
    fn copysign(self, sign: Self) -> Self;

    /// Computes the cosine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0 * std::f32::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn cos(self) -> Self;

    /// Returns the maximum of the two numbers.
    ///
    /// Follows the IEEE-754 2008 semantics for maxNum, except for handling of signaling NaNs.
    /// This matches the behavior of libm’s fmin.
    ///
    /// ```
    /// let x = 1.0f32;
    /// let y = 2.0f32;
    ///
    /// assert_eq!(x.max(y), y);
    /// ```
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers.
    ///
    /// Follows the IEEE-754 2008 semantics for minNum, except for handling of signaling NaNs.
    /// This matches the behavior of libm’s fmin.
    ///
    /// ```
    /// let x = 1.0f32;
    /// let y = 2.0f32;
    ///
    /// assert_eq!(x.min(y), x);
    /// ```
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// Computes the sine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn sin(self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 <= f32::EPSILON);
    /// assert!(abs_difference_1 <= f32::EPSILON);
    /// ```
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    fn sqrt(self) -> Self;

    /// Converts radians to degrees.
    ///
    /// ```
    /// let angle = std::f32::consts::PI;
    ///
    /// let abs_difference = (angle.to_degrees() - 180.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn to_degrees(self) -> Self;

    /// Converts degrees to radians.
    ///
    /// ```
    /// let angle = 180.0f32;
    ///
    /// let abs_difference = (angle.to_radians() - std::f32::consts::PI).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn to_radians(self) -> Self;
}

impl const Real for f32 {
    const DIGITS: u32 = core::f32::DIGITS;
    const EPSILON: f32 = core::f32::EPSILON;
    const INFINITY: f32 = core::f32::INFINITY;
    const MANTISSA_DIGITS: u32 = core::f32::MANTISSA_DIGITS;
    const MAX: f32 = core::f32::MAX;
    const MAX_10_EXP: i32 = core::f32::MAX_10_EXP;
    const MAX_EXP: i32 = core::f32::MAX_EXP;
    const MIN: f32 = core::f32::MIN;
    const MIN_10_EXP: i32 = core::f32::MIN_10_EXP;
    const MIN_EXP: i32 = core::f32::MIN_EXP;
    const NAN: f32 = core::f32::NAN;
    const NEG_INFINITY: f32 = core::f32::NEG_INFINITY;

    const E: f32 = ::core::f32::consts::E;
    const FRAC_1_PI: f32 = ::core::f32::consts::FRAC_1_PI;
    const FRAC_1_SQRT_2: f32 = ::core::f32::consts::FRAC_1_SQRT_2;
    const FRAC_2_PI: f32 = ::core::f32::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: f32 = ::core::f32::consts::FRAC_2_SQRT_PI;
    const FRAC_PI_2: f32 = ::core::f32::consts::FRAC_PI_2;
    const FRAC_PI_3: f32 = ::core::f32::consts::FRAC_PI_3;
    const FRAC_PI_4: f32 = ::core::f32::consts::FRAC_PI_4;
    const FRAC_PI_6: f32 = ::core::f32::consts::FRAC_PI_6;
    const FRAC_PI_8: f32 = ::core::f32::consts::FRAC_PI_8;
    const LN_2: f32 = ::core::f32::consts::LN_2;
    const LN_10: f32 = ::core::f32::consts::LN_10;
    const LOG2_10: f32 = ::core::f32::consts::LOG2_10;
    const LOG2_E: f32 = ::core::f32::consts::LOG2_E;
    const LOG10_2: f32 = ::core::f32::consts::LOG10_2;
    const LOG10_E: f32 = ::core::f32::consts::LOG10_E;
    const PI: f32 = <Self as Sealed>::_PI;
    const SQRT_2: f32 = ::core::f32::consts::SQRT_2;
    const TAU: f32 = ::core::f32::consts::TAU;

    #[must_use]
    #[inline]
    fn abs(self) -> f32 {
        libm::fabsf(self)
    }

    #[must_use]
    #[inline]
    fn asin(self) -> f32 {
        libm::asinf(self)
    }

    #[must_use]
    #[inline]
    fn atan2(self, other: f32) -> f32 {
        libm::atan2f(self, other)
    }

    #[must_use]
    #[inline]
    fn clamp(self, min: f32, max: f32) -> f32 {
        let mut x = self;

        if x < min {
            x = min;
        }

        if x > max {
            x = max;
        }

        x
    }

    #[must_use]
    #[inline]
    fn copysign(self, sign: f32) -> f32 {
        libm::copysignf(self, sign)
    }

    #[must_use]
    #[inline]
    fn cos(self) -> f32 {
        libm::cosf(self)
    }

    #[must_use]
    #[inline]
    fn max(self, other: f32) -> f32 {
        libm::fmaxf(self, other)
    }

    #[must_use]
    #[inline]
    fn min(self, other: f32) -> f32 {
        libm::fminf(self, other)
    }

    #[must_use]
    #[inline]
    fn sin(self) -> f32 {
        libm::sinf(self)
    }

    #[must_use]
    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        libm::sincosf(self)
    }

    #[must_use]
    #[inline]
    fn sqrt(self) -> f32 {
        libm::sqrtf(self)
    }

    #[must_use]
    #[inline]
    fn to_degrees(self) -> f32 {
        self * <f32 as Sealed>::_180_PI
    }

    #[must_use]
    #[inline]
    fn to_radians(self) -> f32 {
        self * <f32 as Sealed>::_PI_180
    }
}

impl const Real for f64 {
    const DIGITS: u32 = f64::DIGITS;
    const EPSILON: f64 = f64::EPSILON;
    const INFINITY: f64 = f64::INFINITY;
    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    const MAX: f64 = f64::MAX;
    const MAX_10_EXP: i32 = f64::MAX_10_EXP;
    const MAX_EXP: i32 = f64::MAX_EXP;
    const MIN: f64 = f64::MIN;
    const MIN_10_EXP: i32 = f64::MIN_10_EXP;
    const MIN_EXP: i32 = f64::MIN_EXP;
    const NAN: f64 = f64::NAN;
    const NEG_INFINITY: f64 = f64::NEG_INFINITY;

    const E: f64 = ::core::f64::consts::E;
    const FRAC_1_PI: f64 = ::core::f64::consts::FRAC_1_PI;
    const FRAC_1_SQRT_2: f64 = ::core::f64::consts::FRAC_1_SQRT_2;
    const FRAC_2_PI: f64 = ::core::f64::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: f64 = ::core::f64::consts::FRAC_2_SQRT_PI;
    const FRAC_PI_2: f64 = ::core::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: f64 = ::core::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: f64 = ::core::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: f64 = ::core::f64::consts::FRAC_PI_6;
    const FRAC_PI_8: f64 = ::core::f64::consts::FRAC_PI_8;
    const LN_2: f64 = ::core::f64::consts::LN_2;
    const LN_10: f64 = ::core::f64::consts::LN_10;
    const LOG2_10: f64 = ::core::f64::consts::LOG2_10;
    const LOG2_E: f64 = ::core::f64::consts::LOG2_E;
    const LOG10_2: f64 = ::core::f64::consts::LOG10_2;
    const LOG10_E: f64 = ::core::f64::consts::LOG10_E;
    const PI: f64 = <Self as Sealed>::_PI;
    const SQRT_2: f64 = ::core::f64::consts::SQRT_2;
    const TAU: f64 = ::core::f64::consts::TAU;

    #[must_use]
    #[inline]
    fn abs(self) -> f64 {
        libm::fabs(self)
    }

    #[must_use]
    #[inline]
    fn asin(self) -> f64 {
        libm::asin(self)
    }

    #[must_use]
    #[inline]
    fn atan2(self, other: f64) -> f64 {
        libm::atan2(self, other)
    }

    #[must_use]
    #[inline]
    fn clamp(self, min: f64, max: f64) -> f64 {
        let mut x = self;

        if x < min {
            x = min;
        }

        if x > max {
            x = max;
        }

        x
    }

    #[must_use]
    #[inline]
    fn copysign(self, sign: f64) -> f64 {
        libm::copysign(self, sign)
    }

    #[must_use]
    #[inline]
    fn cos(self) -> f64 {
        libm::cos(self)
    }

    #[must_use]
    #[inline]
    fn max(self, other: f64) -> f64 {
        libm::fmax(self, other)
    }

    #[must_use]
    #[inline]
    fn min(self, other: f64) -> f64 {
        libm::fmin(self, other)
    }

    #[must_use]
    #[inline]
    fn sin(self) -> f64 {
        libm::sin(self)
    }

    #[must_use]
    #[inline]
    fn sin_cos(self) -> (f64, f64) {
        libm::sincos(self)
    }

    #[must_use]
    #[inline]
    fn sqrt(self) -> f64 {
        libm::sqrt(self)
    }

    #[must_use]
    #[inline]
    fn to_degrees(self) -> f64 {
        self * <f64 as Sealed>::_180_PI
    }

    #[must_use]
    #[inline]
    fn to_radians(self) -> f64 {
        self * <f64 as Sealed>::_PI_180
    }
}
