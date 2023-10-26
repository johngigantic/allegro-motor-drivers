//! Mask configuration register
//!
//! The mask register enables or disables detection of various faults.

use bilge::prelude::*;

use super::AllegroRegister;

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

impl AllegroRegister for Mask {
    fn value(&self) -> u16 {
        self.value.into()
    }

    fn set_value(&mut self, value: u13) {
        self.value = value;
    }
}
