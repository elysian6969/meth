#![feature(const_trait_impl)]
#![feature(inline_const)]

use meth::Vec;

extern "C" {
    fn scanf(fmt: *const u8, ...);
}

fn main() {
    unsafe {
        let mut vec1 = Vec::<i32, 4>::splat(0);
        let mut vec2 = Vec::<i32, 4>::splat(0);

        print!("Enter four numbers seperated by spaces: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        scanf(
            "%d %d %d %d\0".as_ptr(),
            vec1.as_mut_ptr(),
            vec1.as_mut_ptr().add(1),
            vec1.as_mut_ptr().add(2),
            vec1.as_mut_ptr().add(3),
        );

        print!("Enter four more numbers seperated by spaces: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        scanf(
            "%d %d %d %d\0".as_ptr(),
            vec2.as_mut_ptr(),
            vec2.as_mut_ptr().add(1),
            vec2.as_mut_ptr().add(2),
            vec2.as_mut_ptr().add(3),
        );

        println!("{:?} + {:?} = {:?}", vec1, vec2, vec1 + vec2);
    }
}
