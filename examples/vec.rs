#![feature(const_trait_impl)]

use meth::Vec;

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
}
