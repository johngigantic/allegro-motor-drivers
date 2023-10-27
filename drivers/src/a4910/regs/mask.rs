//! Mask configuration register
//!
//! The mask register enables or disables detection of various faults.

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

use crate::regs::ConstantAddress;
use super::A4910Reg;

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits, AllegroRegister)]
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

impl ConstantAddress<A4910Reg> for Mask {
    const ADDRESS: A4910Reg = A4910Reg::Mask;
}