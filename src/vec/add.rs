use super::{Element, LaneCount, Lanes, Vec};
use crate::intrinsics;
use core::intrinsics::const_eval_select;
use core::ops::Add;

#[inline]
#[must_use]
pub const fn add<T, const N: usize>(a: Vec<T, N>, b: Vec<T, N>) -> Vec<T, N>
where
    T: ~const Element,
    T: ~const Add<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    #[must_use]
    const fn scalar_add<T, const N: usize>(mut a: Vec<T, N>, b: Vec<T, N>) -> Vec<T, N>
    where
        T: ~const Element,
        T: ~const Add<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        let mut a_iter = a.iter_mut();
        let mut b_iter = b.iter();

        while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
            *a = *a + *b;
        }

        a
    }

    #[inline]
    #[must_use]
    fn simd_add<T, const N: usize>(a: Vec<T, N>, b: Vec<T, N>) -> Vec<T, N>
    where
        T: Element,
        T: Add<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        unsafe {
            let mut a_iter = a.0.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);
            let mut b_iter = b.0.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);

            while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
                let a = a.as_ptr() as *mut [T; <Lanes<T, N> as LaneCount>::LANES];
                let b = b.as_ptr() as *mut [T; <Lanes<T, N> as LaneCount>::LANES];

                *a = intrinsics::simd_add(*a, *b);
            }

            let mut a_iter = a_iter.remainder().iter();
            let mut b_iter = b_iter.remainder().iter();

            while let (Some(a), Some(b)) = (a_iter.next(), b_iter.next()) {
                let a = &mut *(a as *const T as *mut T);

                *a = *a + *b;
            }

            a
        }
    }

    unsafe { const_eval_select((a, b), scalar_add, simd_add) }
}
