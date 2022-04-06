use super::{Element, LaneCount, Lanes, Vec};
use crate::identity::Zero;
use crate::intrinsics;
use core::intrinsics::const_eval_select;
use core::ops::Add;

#[inline]
#[must_use]
pub const fn sum<T, const N: usize>(vec: Vec<T, N>) -> T
where
    T: ~const Element,
    T: ~const Zero,
    T: ~const Add<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    #[must_use]
    const fn scalar_sum<T, const N: usize>(vec: Vec<T, N>) -> T
    where
        T: ~const Element,
        T: ~const Zero,
        T: ~const Add<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        let mut output = <T as Zero>::zero();
        let mut iter = vec.iter();

        while let Some(element) = iter.next() {
            output = output + *element;
        }

        output
    }

    #[inline]
    #[must_use]
    fn simd_sum<T, const N: usize>(vec: Vec<T, N>) -> T
    where
        T: Element,
        T: Zero,
        T: Add<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        unsafe {
            let mut output = <T as Zero>::zero();
            let mut iter = vec.array.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);

            while let Some(element) = iter.next() {
                let element = element.as_ptr() as *const [T; <Lanes<T, N> as LaneCount>::LANES];

                output = output + intrinsics::simd_sum(*element);
            }

            let mut iter = iter.remainder().iter();

            while let Some(element) = iter.next() {
                output = output + *element;
            }

            output
        }
    }

    unsafe { const_eval_select((vec,), scalar_sum, simd_sum) }
}
