//! Mask register settings

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[derive(AllegroRegister)]
#[bitsize(12)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Mask {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u1,
    pub vs: bool,
    reserved: u1,
    pub los: bool,
    pub ot: bool,
    pub tw: bool,
}
