//! Run configuration register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

use super::A4910Reg;
use crate::regs::ConstantAddress;

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits, AllegroRegister)]
pub struct Run {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u7,
}

impl ConstantAddress<A4910Reg> for Run {
    const ADDRESS: A4910Reg = A4910Reg::Run;
}
