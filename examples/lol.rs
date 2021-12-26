#![feature(inline_const)]

use meth::{EulerAngles, Quaternion};

fn main() {
    println!(
        "{:?}",
        const { Quaternion::<f32>::from_xyzw(0.5, 5.0, 5.0, 0.5).into_euler_angles() }
    );
}
