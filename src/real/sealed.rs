pub trait Sealed: Sized {
    const _PI: Self;
    const _180_PI: Self;
    const _PI_180: Self;
}

impl const Sealed for f32 {
    const _PI: f32 = ::core::f32::consts::PI;
    #[allow(clippy::excessive_precision)]
    const _180_PI: f32 = 57.2957795130823208767981548141051703;
    const _PI_180: f32 = Self::_PI / 180.0;
}

impl const Sealed for f64 {
    const _PI: f64 = ::core::f64::consts::PI;
    const _180_PI: f64 = 180.0 / Self::_PI;
    const _PI_180: f64 = Self::_PI / 180.0;
}
