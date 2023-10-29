//! Mask register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct Mask {
    pub vo: bool,
    pub bu: bool,
    pub vlu: bool,
    pub vru: bool,
    pub vsu: bool,
    pub tw: bool,
    pub ot: bool,
    pub los: bool,
    pub wd: bool,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            value: u9::new(0b1_0_0_0_0_0_0_0_0),
        }
    }
}
