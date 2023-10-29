//! Mask configuration register
//!
//! The mask register enables or disables detection of various faults.

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[derive(AllegroRegister)]
#[bitsize(13)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
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
