//! Startup registers 0-5

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct AlignmentTime(u4);

impl Default for AlignmentTime {
    fn default() -> Self {
        Self {
            value: u4::new(0b0001),
        }
    }
}

#[bitsize(5)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct AlignmentPeakPwmDuty(u5);

impl Default for AlignmentPeakPwmDuty {
    fn default() -> Self {
        Self {
            value: u5::new(0b00101),
        }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum StartCoastMode {
    #[default]
    CoastDisabled,
    CoastEnabled,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum RestartMode {
    #[default]
    NoRestart,
    AllowRestart,
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct MotorConstant(u4);

impl Default for MotorConstant {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum AlignmentDutyCycleRampTime {
    #[default]
    Percent0,
    Percent25,
    Percent50,
    Percent100,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum WindmillMode {
    #[default]
    WindmillingDisabled,
    WindmillingEnabled,
}

#[bitsize(3)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct MinWindmillDetectionFrequency(u3);

impl Default for MinWindmillDetectionFrequency {
    fn default() -> Self {
        Self {
            value: u3::new(0b010),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct WindmillBrakingDutyCycle(u4);

impl Default for WindmillBrakingDutyCycle {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct StartRampFinalFrequency(u4);

impl Default for StartRampFinalFrequency {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct StartRampInitialFrequency(u4);

impl Default for StartRampInitialFrequency {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct StartRampFinalDutyCycle(u4);

impl Default for StartRampFinalDutyCycle {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct StartRampInitialDutyCycle(u4);

impl Default for StartRampInitialDutyCycle {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct StartRampStepTime(u4);

impl Default for StartRampStepTime {
    fn default() -> Self {
        Self {
            value: u4::new(0b0100),
        }
    }
}

#[bitsize(4)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum StartRampFrequencyStep {
    Hz0f0125,
    Hz0f025,
    Hz0f5,
    Hz0f1,
    Hz0f2,
    Hz0f4,
    Hz0f8,
    #[default]
    Hz1,
    Hz1f5,
    Hz2,
    Hz2f5,
    Hz3,
    Hz5,
    Hz8,
    Hz10,
    Hz15,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup0 {
    pub hd: AlignmentPeakPwmDuty,
    pub ht: AlignmentTime,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup1 {
    pub hr: AlignmentDutyCycleRampTime,
    pub km: MotorConstant,
    reserved: u1,
    pub rsc: RestartMode,
    pub stm: StartCoastMode,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup2 {
    pub wbd: WindmillBrakingDutyCycle,
    pub wmf: MinWindmillDetectionFrequency,
    pub win: WindmillMode,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup3 {
    pub sf1: StartRampInitialFrequency,
    pub sf2: StartRampFinalFrequency,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup4 {
    pub sd1: StartRampInitialDutyCycle,
    pub sd2: StartRampFinalDutyCycle,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Startup5 {
    pub sfs: StartRampFrequencyStep,
    pub sts: StartRampStepTime,
    reserved: u1,
}

pub type Startup = (Startup0, Startup1, Startup2, Startup3, Startup4, Startup5);
