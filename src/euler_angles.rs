use crate::{Quaternion, Real};

#[derive(Clone, Copy, Debug)]
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

impl const From<Quaternion<f32>> for EulerAngles<f32> {
    fn from(quarernion: Quaternion<f32>) -> Self {
        // roll (x-axis rotation)
        let sin_roll_cos_pitch = 2.0 * (quarernion.w * quarernion.x + quarernion.y * quarernion.z);
        let cos_roll_cos_pitch =
            1.0 - 2.0 * (quarernion.x * quarernion.x + quarernion.y * quarernion.y);
        let roll = Real::atan2(sin_roll_cos_pitch, cos_roll_cos_pitch);

        // pitch (z-axis rotation)
        let sin_pitch = 2.0 * (quarernion.w * quarernion.y - quarernion.z * quarernion.x);
        let pitch = if Real::abs(sin_pitch) >= 1.0 {
            // use 90 degrees if out of range
            Real::copysign(Real::FRAC_PI_2, sin_pitch)
        } else {
            Real::asin(sin_pitch)
        };

        // yaw (z-axis rotation)
        let sin_yaw_cos_pitch = 2.0 * (quarernion.w * quarernion.z + quarernion.x * quarernion.y);
        let cos_yaw_cos_pitch =
            1.0 - 2.0 * (quarernion.y * quarernion.y + quarernion.z * quarernion.z);
        let yaw = Real::atan2(sin_yaw_cos_pitch, cos_yaw_cos_pitch);

        EulerAngles { roll, yaw, pitch }
    }
}

impl const From<Quaternion<f64>> for EulerAngles<f64> {
    fn from(quarernion: Quaternion<f64>) -> Self {
        // roll (x-axis rotation)
        let sin_roll_cos_pitch = 2.0 * (quarernion.w * quarernion.x + quarernion.y * quarernion.z);
        let cos_roll_cos_pitch =
            1.0 - 2.0 * (quarernion.x * quarernion.x + quarernion.y * quarernion.y);
        let roll = Real::atan2(sin_roll_cos_pitch, cos_roll_cos_pitch);

        // pitch (z-axis rotation)
        let sin_pitch = 2.0 * (quarernion.w * quarernion.y - quarernion.z * quarernion.x);
        let pitch = if Real::abs(sin_pitch) >= 1.0 {
            // use 90 degrees if out of range
            Real::copysign(Real::FRAC_PI_2, sin_pitch)
        } else {
            Real::asin(sin_pitch)
        };

        // yaw (z-axis rotation)
        let sin_yaw_cos_pitch = 2.0 * (quarernion.w * quarernion.z + quarernion.x * quarernion.y);
        let cos_yaw_cos_pitch =
            1.0 - 2.0 * (quarernion.y * quarernion.y + quarernion.z * quarernion.z);
        let yaw = Real::atan2(sin_yaw_cos_pitch, cos_yaw_cos_pitch);

        EulerAngles { roll, yaw, pitch }
    }
}
