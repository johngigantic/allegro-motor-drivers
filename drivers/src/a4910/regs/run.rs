//! Run configuration register

use bilge::prelude::*;

use crate::regs::{AllegroRegister, ConstantAddress};
use super::A4910Reg;

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct Run {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u7,
}

impl AllegroRegister<u13> for Run {
    fn get_value(&self) -> u16 {
        self.value.into()
    }

    fn set_value(&mut self, value: u13) {
        self.value = value;
    }
}

impl ConstantAddress<A4910Reg> for Run {
    const ADDRESS: A4910Reg = A4910Reg::Run;
}
