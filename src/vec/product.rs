use super::{Element, LaneCount, Lanes, Vec};
use crate::identity::One;
use crate::intrinsics;
use core::intrinsics::const_eval_select;
use core::ops::Mul;

#[inline]
#[must_use]
pub const fn product<T, const N: usize>(vec: Vec<T, N>) -> T
where
    T: ~const Element,
    T: ~const One,
    T: ~const Mul<Output = T>,
    Lanes<T, N>: LaneCount,
    [(); Lanes::<T, N>::LANES]:,
{
    #[inline]
    #[must_use]
    const fn scalar_product<T, const N: usize>(vec: Vec<T, N>) -> T
    where
        T: ~const Element,
        T: ~const One,
        T: ~const Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        let mut output = <T as One>::one();
        let mut iter = vec.iter();

        while let Some(element) = iter.next() {
            output = output * *element;
        }

        output
    }

    #[inline]
    #[must_use]
    fn simd_product<T, const N: usize>(vec: Vec<T, N>) -> T
    where
        T: Element,
        T: One,
        T: Mul<Output = T>,
        Lanes<T, N>: LaneCount,
        [(); Lanes::<T, N>::LANES]:,
    {
        unsafe {
            let mut output = <T as One>::one();
            let mut iter = vec.0.chunks_exact(<Lanes<T, N> as LaneCount>::LANES);

            while let Some(element) = iter.next() {
                let element = element.as_ptr() as *const [T; <Lanes<T, N> as LaneCount>::LANES];

                output = output * intrinsics::simd_product(*element);
            }

            let mut iter = iter.remainder().iter();

            while let Some(element) = iter.next() {
                output = output * *element;
            }

            output
        }
    }

    unsafe { const_eval_select((vec,), scalar_product, simd_product) }
}
