#![allow(incomplete_features)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]

pub use self::consts::Consts;
pub use self::float::Float;
pub use self::one::One;
pub use self::zero::Zero;

mod consts;
mod float;
mod one;
mod zero;

#[derive(Debug)]
#[repr(C)]
pub struct Quaternion<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Quaternion<T> {
    pub const fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub const fn zero() -> Self
    where
        T: ~const Zero,
    {
        Self {
            x: <T as Zero>::zero(),
            y: <T as Zero>::zero(),
            z: <T as Zero>::zero(),
            w: <T as Zero>::zero(),
        }
    }

    pub const fn one() -> Self
    where
        T: ~const One,
    {
        Self {
            x: <T as One>::one(),
            y: <T as One>::one(),
            z: <T as One>::one(),
            w: <T as One>::one(),
        }
    }

    // @note: https://wikiless.org/wiki/Conversion_between_quaternions_and_Euler_angles
    pub const fn from_euler_angles(angles: EulerAngles<T>) -> Self
    where
        Self: ~const FromEulerAngles<T>,
    {
        FromEulerAngles::from_euler_angles(angles)
    }

    pub const fn into_euler_angles(self) -> EulerAngles<T>
    where
        Self: ~const IntoEulerAngles<T>,
    {
        IntoEulerAngles::into_euler_angles(self)
    }
}

pub trait FromEulerAngles<T> {
    fn from_euler_angles(angles: EulerAngles<T>) -> Self;
}

impl const FromEulerAngles<f32> for Quaternion<f32> {
    fn from_euler_angles(angles: EulerAngles<f32>) -> Self {
        let yaw = angles.yaw * 0.5;
        let pitch = angles.pitch * 0.5;
        let roll = angles.roll * 0.5;

        let (sin_yaw, cos_yaw) = Float::sin_cos(yaw);
        let (sin_pitch, cos_pitch) = Float::sin_cos(pitch);
        let (sin_roll, cos_roll) = Float::sin_cos(roll);

        let w = cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw;
        let x = sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw;
        let y = cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw;
        let z = cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw;

        Self { x, y, z, w }
    }
}

impl const FromEulerAngles<f64> for Quaternion<f64> {
    fn from_euler_angles(angles: EulerAngles<f64>) -> Self {
        let yaw = angles.yaw * 0.5;
        let pitch = angles.pitch * 0.5;
        let roll = angles.roll * 0.5;

        let (sin_yaw, cos_yaw) = Float::sin_cos(yaw);
        let (sin_pitch, cos_pitch) = Float::sin_cos(pitch);
        let (sin_roll, cos_roll) = Float::sin_cos(roll);

        let w = cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw;
        let x = sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw;
        let y = cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw;
        let z = cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw;

        Self { x, y, z, w }
    }
}

pub trait IntoEulerAngles<T> {
    fn into_euler_angles(self) -> EulerAngles<T>;
}

impl const IntoEulerAngles<f32> for Quaternion<f32> {
    fn into_euler_angles(self) -> EulerAngles<f32> {
        // roll (x-axis rotation)
        let sin_roll_cos_pitch = 2.0 * (self.w * self.x + self.y * self.z);
        let cos_roll_cos_pitch = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = Float::atan2(sin_roll_cos_pitch, cos_roll_cos_pitch);

        // pitch (z-axis rotation)
        let sin_pitch = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if Float::abs(sin_pitch) >= 1.0 {
            // use 90 degrees if out of range
            Float::copysign(<f32 as Consts>::FRAC_PI_2, sin_pitch)
        } else {
            Float::asin(sin_pitch)
        };

        // yaw (z-axis rotation)
        let sin_yaw_cos_pitch = 2.0 * (self.w * self.z + self.x * self.y);
        let cos_yaw_cos_pitch = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = Float::atan2(sin_yaw_cos_pitch, cos_yaw_cos_pitch);

        EulerAngles { pitch, yaw, roll }
    }
}

impl const IntoEulerAngles<f64> for Quaternion<f64> {
    fn into_euler_angles(self) -> EulerAngles<f64> {
        // roll (x-axis rotation)
        let sin_roll_cos_pitch = 2.0 * (self.w * self.x + self.y * self.z);
        let cos_roll_cos_pitch = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = Float::atan2(sin_roll_cos_pitch, cos_roll_cos_pitch);

        // pitch (z-axis rotation)
        let sin_pitch = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if Float::abs(sin_pitch) >= 1.0 {
            // use 90 degrees if out of range
            Float::copysign(<f64 as Consts>::FRAC_PI_2, sin_pitch)
        } else {
            Float::asin(sin_pitch)
        };

        // yaw (z-axis rotation)
        let sin_yaw_cos_pitch = 2.0 * (self.w * self.z + self.x * self.y);
        let cos_yaw_cos_pitch = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = Float::atan2(sin_yaw_cos_pitch, cos_yaw_cos_pitch);

        EulerAngles { pitch, yaw, roll }
    }
}

#[derive(Debug)]
pub struct EulerAngles<T> {
    pub pitch: T,
    pub yaw: T,
    pub roll: T,
}

impl<T> EulerAngles<T> {
    pub const fn new(pitch: T, yaw: T, roll: T) -> Self {
        Self { pitch, yaw, roll }
    }
}

#[repr(C)]
pub struct Matrix<T, const M: usize, const N: usize>([T; M * N])
where
    [(); M * N]:;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    [(); M * N]:,
{
    pub const fn zero() -> Self
    where
        T: ~const Zero + Copy,
    {
        Self([<T as Zero>::zero(); M * N])
    }

    pub const fn one() -> Self
    where
        T: ~const One + Copy,
    {
        Self([<T as One>::one(); M * N])
    }
}
