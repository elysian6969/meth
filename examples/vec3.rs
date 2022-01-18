#![feature(const_trait_impl)]

use meth::Vec3;

fn main() {
    let a = Vec3::from_array([1, 2, 3]);
    let b = Vec3::from_array([3, 2, 1]);

    println!("{:?}", a + b);
}
