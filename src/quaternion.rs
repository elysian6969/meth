use crate::{EulerAngles, One, Real, Zero};

#[derive(Clone, Copy, Debug)]
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

    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    pub const fn zero() -> Self
    where
        T: ~const Zero,
    {
        Self {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
            w: Zero::zero(),
        }
    }

    pub const fn one() -> Self
    where
        T: ~const One,
    {
        Self {
            x: One::one(),
            y: One::one(),
            z: One::one(),
            w: One::one(),
        }
    }
}

// @note: https://wikiless.org/wiki/Conversion_between_quaternions_and_Euler_angles
impl const From<EulerAngles<f32>> for Quaternion<f32> {
    fn from(angles: EulerAngles<f32>) -> Self {
        let yaw = angles.yaw * 0.5;
        let pitch = angles.pitch * 0.5;
        let roll = angles.roll * 0.5;

        let (sin_yaw, cos_yaw) = Real::sin_cos(yaw);
        let (sin_pitch, cos_pitch) = Real::sin_cos(pitch);
        let (sin_roll, cos_roll) = Real::sin_cos(roll);

        let w = cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw;
        let x = sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw;
        let y = cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw;
        let z = cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw;

        Self { x, y, z, w }
    }
}

// @note: https://wikiless.org/wiki/Conversion_between_quaternions_and_Euler_angles
impl const From<EulerAngles<f64>> for Quaternion<f64> {
    fn from(angles: EulerAngles<f64>) -> Self {
        let yaw = angles.yaw * 0.5;
        let pitch = angles.pitch * 0.5;
        let roll = angles.roll * 0.5;

        let (sin_yaw, cos_yaw) = Real::sin_cos(yaw);
        let (sin_pitch, cos_pitch) = Real::sin_cos(pitch);
        let (sin_roll, cos_roll) = Real::sin_cos(roll);

        let w = cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw;
        let x = sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw;
        let y = cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw;
        let z = cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw;

        Self { x, y, z, w }
    }
}
