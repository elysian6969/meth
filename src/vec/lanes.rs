mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

pub struct Lanes<const LANES: usize>;

pub trait SupportedLanes: Sealed {}

impl<const LANES: usize> const Sealed for Lanes<LANES> {}

impl const SupportedLanes for Lanes<1> {}
impl const SupportedLanes for Lanes<2> {}
impl const SupportedLanes for Lanes<4> {}
impl const SupportedLanes for Lanes<8> {}
impl const SupportedLanes for Lanes<16> {}
impl const SupportedLanes for Lanes<32> {}
impl const SupportedLanes for Lanes<64> {}
