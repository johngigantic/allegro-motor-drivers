//! Non-volatile memory write register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(2)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct SaveParameters(u2);

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct NvmWrite {
    reserved: u7,
    pub sav: SaveParameters,
}
