pub trait Consts {
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
}

impl const Consts for f32 {
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
    const PI: f32 = ::core::f32::consts::PI;
    const SQRT_2: f32 = ::core::f32::consts::SQRT_2;
    const TAU: f32 = ::core::f32::consts::TAU;
}

impl const Consts for f64 {
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
    const PI: f64 = ::core::f64::consts::PI;
    const SQRT_2: f64 = ::core::f64::consts::SQRT_2;
    const TAU: f64 = ::core::f64::consts::TAU;
}
