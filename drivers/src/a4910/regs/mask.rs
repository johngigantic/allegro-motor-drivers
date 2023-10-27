//! Mask configuration register
//!
//! The mask register enables or disables detection of various faults.

use bilge::prelude::*;

use crate::regs::{AllegroRegister, ConstantAddress};
use super::A4910Reg;

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct Mask {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    pub vc: bool,
    pub vb: bool,
    pub va: bool,
    pub vr: bool,
    pub ot: bool,
    pub tw: bool,
    reserved: u1,
}

impl AllegroRegister<u13> for Mask {
    fn get_value(&self) -> u16 {
        self.value.into()
    }

    fn set_value(&mut self, value: u13) {
        self.value = value;
    }
}

impl ConstantAddress<A4910Reg> for Mask {
    const ADDRESS: A4910Reg = A4910Reg::Mask;
}