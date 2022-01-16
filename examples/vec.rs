#![feature(const_trait_impl)]

use meth::{Real, Vec};

fn main() {
    let vec = Vec::from_array([
        1_u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);

    println!("vec + vec = {:?}", vec + vec);
    println!("vec / vec = {:?}", vec / vec);
    println!("vec * vec = {:?}", vec * vec);
    println!("vec % vec = {:?}", vec % vec);
    println!("vec - vec = {:?}", vec - vec);

    let vec = Vec::from_array([1_u32, 2, 3]);

    println!("vec + vec = {:?}", vec + vec);

    const CONSTS: [Vec<u32, 18>; 5] = {
        let vec = Vec::from_array([
            1_u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ]);

        [vec + vec, vec / vec, vec * vec, vec % vec, vec - vec]
    };

    println!("vec + vec = {:?}", CONSTS[0]);
    println!("vec / vec = {:?}", CONSTS[1]);
    println!("vec * vec = {:?}", CONSTS[2]);
    println!("vec % vec = {:?}", CONSTS[3]);
    println!("vec - vec = {:?}", CONSTS[4]);

    let vec = Vec::from_array([1.0_f32, 2.0, 3.0]);

    println!("vec.distance() = {:?}", vec.distance(vec));
    println!("vec.distance_squared() = {:?}", vec.distance_squared(vec));

    println!("vec.magnitude() = {:?}", vec.magnitude());
    println!("vec.magnitude_squared() = {:?}", vec.magnitude_squared());

    println!("vec.product() = {:?}", vec.product());
    println!("vec.sum() = {:?}", vec.sum());

    println!(
        "vec.to_degrees() = {:?}",
        Vec::from_array([
            <f32 as Real>::PI * 0.0,
            <f32 as Real>::PI * 0.5,
            <f32 as Real>::PI * 1.0,
            <f32 as Real>::PI * 1.5,
            <f32 as Real>::PI * 2.0,
        ])
        .to_degrees(),
    );
}
