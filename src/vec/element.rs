mod sealed {
    pub trait Sealed: Copy + Sized {}
}

use sealed::Sealed;

/// SIMD-able types.
pub trait Element: Sealed {}

macro_rules! impl_element {
    { $ty:ty } => {
        impl const sealed::Sealed for $ty {}
        impl const Element for $ty {}
    }
}

impl_element! { i8 }
impl_element! { i16 }
impl_element! { i32 }
impl_element! { i64 }
impl_element! { isize }

impl_element! { u8 }
impl_element! { u16 }
impl_element! { u32 }
impl_element! { u64 }
impl_element! { usize }

impl_element! { f32 }
impl_element! { f64 }
