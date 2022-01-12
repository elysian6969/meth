mod sealed {
    pub trait Sealed: Sized {}

    impl const Sealed for i8 {}
    impl const Sealed for i16 {}
    impl const Sealed for i32 {}
    impl const Sealed for i64 {}
    impl const Sealed for isize {}

    impl const Sealed for u8 {}
    impl const Sealed for u16 {}
    impl const Sealed for u32 {}
    impl const Sealed for u64 {}
    impl const Sealed for usize {}

    impl const Sealed for f32 {}
    impl const Sealed for f64 {}
}

use self::sealed::Sealed;

/// SIMD-able types.
pub trait Element: Sealed {}

impl const Element for i8 {}
impl const Element for i16 {}
impl const Element for i32 {}
impl const Element for i64 {}
impl const Element for isize {}

impl const Element for u8 {}
impl const Element for u16 {}
impl const Element for u32 {}
impl const Element for u64 {}
impl const Element for usize {}

impl const Element for f32 {}
impl const Element for f64 {}
