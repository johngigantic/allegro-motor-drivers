//! Read Only register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(10)]
#[derive(DebugBits, PartialEq, PartialOrd, DefaultBits, FromBits, Clone, Copy)]
pub struct DiagnosticRegister {
    pub osr: bool,
    pub ba: bool,
    pub bb: bool,
    pub bc: bool,
    pub ah: bool,
    pub al: bool,
    pub bh: bool,
    pub bl: bool,
    pub ch: bool,
    pub cl: bool,
}

#[derive(AllegroRegister)]
#[bitsize(10)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct ReadOnly {
    pub readback: u10,
}
