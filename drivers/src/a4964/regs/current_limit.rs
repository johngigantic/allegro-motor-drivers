//! Current limit register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(5)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct CurrentLimitBlankTime(u5);

impl Default for CurrentLimitBlankTime {
    fn default() -> Self {
        Self {
            value: u5::new(0b00111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct CurrentLimitScale(u4);

impl Default for CurrentLimitScale {
    fn default() -> Self {
        Self {
            value: u4::new(0b1111),
        }
    }
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct CurrentLimit {
    pub vil: CurrentLimitScale,
    pub obt: CurrentLimitBlankTime,
}
