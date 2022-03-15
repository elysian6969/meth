use meth::intrinsics;

unsafe fn foo() {
    let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b: [u32; 8] = [1, 25, 3, 25, 5, 25, 7, 25];
    let c = intrinsics::simd_eq(a, b);

    println!("{c:?}");

    let c = intrinsics::simd_ne(a, b);

    println!("{c:?}");

    let a: [u32; 2] = [1, 2];
    let b: [u32; 2] = [2, 1];

    let c = intrinsics::simd_lt(a, b);

    println!("{c:?}");

    let c: [bool; 2] = [1 < 2, 2 < 1];

    println!("{c:?}");
}

fn main() {
    unsafe {
        foo();
    }
}
