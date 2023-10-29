//! Pulsed width modulation registers 0-1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum ModulationMode {
    #[default]
    Phase3,
    Phase2,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum BridgePwmMode {
    #[default]
    CenterAligned,
    EdgeAligned,
}

#[bitsize(6)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct BridgePwmPeriod(u6);

impl Default for BridgePwmPeriod {
    fn default() -> Self {
        Self {
            value: u6::new(0b10_0110),
        }
    }
}

#[bitsize(3)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct PwmDitherStepPeriod(u3);

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum PwmDitherDwellTime {
    #[default]
    Ms1,
    Ms2,
    Ms5,
    Ms10,
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct PwmDitherStepCount(u4);

impl From<u8> for PwmDitherStepCount {
    fn from(value: u8) -> Self {
        if value < 16 {
            return Self {
                value: u4::new(value),
            };
        }
        Self { value: u4::new(15) }
    }
}

/// PWM configuration register 0
#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Pwm0 {
    /// Note: the documentation refers to modulation mode as 'mod'.
    /// This variable name is also a Rust language keyword, so it is replaced with 'mmd.'
    pub pw: BridgePwmPeriod,
    pub pmd: BridgePwmMode,
    reserved: u1,
    pub mmd: ModulationMode,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Pwm1 {
    pub ds: PwmDitherStepCount,
    pub dd: PwmDitherDwellTime,
    pub dp: PwmDitherStepPeriod,
}

pub type Pwm = (Pwm0, Pwm1);
