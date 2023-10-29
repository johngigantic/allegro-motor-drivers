//! Bridge register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(2)]
#[derive(Copy, Clone, Debug, PartialEq, Default, FromBits)]
pub enum SenseAmpGain {
    #[default]
    Gain2f5,
    Gain5,
    Gain10,
    Gain20,
}

#[bitsize(6)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct DeadTime(u6);

impl Default for DeadTime {
    fn default() -> Self {
        Self {
            value: u6::new(0b10_0000),
        }
    }
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Bridge {
    pub dt: DeadTime,
    pub sa: SenseAmpGain,
    reserved: u1,
}
