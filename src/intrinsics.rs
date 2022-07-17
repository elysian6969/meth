use cake::mem;

// arithmetic

mod add;
mod div;
mod mul;
mod rem;
mod sub;

pub use add::simd_add;
pub use div::simd_div;
pub use mul::simd_mul;
pub use rem::simd_rem;
pub use sub::simd_sub;

// cast

mod cast;

pub use cast::{simd_cast, simd_cast_mask};

// use an identity (one, zero)

mod product;
mod sum;

pub use product::simd_product;
pub use sum::simd_sum;

// bitwise ops

mod and;

pub use and::simd_and;

// logic ops

mod eq;
mod ge;
mod gt;
mod le;
mod lt;
mod ne;

pub use eq::simd_eq;
pub use ge::simd_ge;
pub use gt::simd_gt;
pub use le::simd_le;
pub use lt::simd_lt;
pub use ne::simd_ne;

#[derive(Clone, Copy)]
#[repr(simd)]
struct Simd<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Simd<T, N> {
    #[inline]
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    #[inline]
    pub const fn to_array(self) -> [T; N]
    where
        T: Copy,
    {
        // SAFETY: it's either this or llvm crashing
        unsafe { mem::transmute(self) }
    }
}

/*use cake::array;

const fn perform_fold<P, T, const N: usize>(
    (left, right, perform): (&[T; N], &[T; N], &P),
    index: usize,
) -> T
where
    P: Copy,
    P: ~const FnOnce(T, T) -> T,
    T: Copy,
{
    perform(left[index], right[index])
}

const fn naive_perform<P, T, const N: usize>(left: [T; N], right: [T; N], perform: P) -> [T; N]
where
    P: Copy,
    P: ~const FnOnce(T, T) -> T,
    T: Copy,
{
    array::fold((&left, &right, &perform), perform_fold)
}*/

//fn simd_perform<T, const N: usize>(left: [T; N], right: [T; N]) -> [T; N] {}
