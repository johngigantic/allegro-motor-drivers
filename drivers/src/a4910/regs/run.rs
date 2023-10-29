//! Run configuration register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[derive(AllegroRegister)]
#[bitsize(13)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Run {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u7,
}
