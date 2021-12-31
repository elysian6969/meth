use meth::Vec;

fn main() {
    let vec = Vec::from_array([
        1_u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);

    println!("{:?}", vec + vec);
    println!("{:?}", vec / vec);
    println!("{:?}", vec * vec);
    println!("{:?}", vec % vec);
    println!("{:?}", vec - vec);
}
