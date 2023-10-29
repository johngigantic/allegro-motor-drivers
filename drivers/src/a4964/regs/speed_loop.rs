//! Speed control loop registers 0-2

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(5)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct SpeedControlAccelerationLimit(u5);

impl Default for SpeedControlAccelerationLimit {
    fn default() -> Self {
        Self {
            value: u5::new(0b00101),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct SpeedControlGain(u4);

impl Default for SpeedControlGain {
    fn default() -> Self {
        Self {
            value: u4::new(0b0101),
        }
    }
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum NominalSupplyVoltage {
    Disabled,
    #[default]
    #[fallback]
    V24,
    V12,
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum DecelerationFactor {
    #[default]
    Factor1,
    Factor2,
    Factor5,
    Factor10,
}

#[bitsize(3)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum SpeedResolution {
    #[default]
    Hz0f1,
    Hz0f2,
    Hz0f4,
    Hz0f8,
    Hz1f6,
    #[fallback]
    Hz3f2,
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct LowUnderspeedThreshold(u4);

impl Default for LowUnderspeedThreshold {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct HighUnderspeedTheshold(u4);

impl Default for HighUnderspeedTheshold {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct SpeedLoop0 {
    pub sg: SpeedControlGain,
    pub sgl: SpeedControlAccelerationLimit,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct SpeedLoop1 {
    pub sr: SpeedResolution,
    reserved: u2,
    pub df: DecelerationFactor,
    pub dv: NominalSupplyVoltage,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct SpeedLoop2 {
    pub sh: HighUnderspeedTheshold,
    pub sl: LowUnderspeedThreshold,
    reserved: u1,
}

pub type SpeedLoop = (SpeedLoop0, SpeedLoop1, SpeedLoop2);
