//! Configuration registers 0 and 1

extern crate allegro_motor_derive;

use bilge::prelude::*;

use crate::regs::AllegroRegister;

#[bitsize(6)]
#[derive(Clone, Copy, DebugBits, PartialEq, FromBits)]
pub struct FaultBlankingTime(u6);

impl Default for FaultBlankingTime {
    fn default() -> Self {
        Self {
            value: u6::new(0b10_0000),
        }
    }
}

#[bitsize(7)]
#[derive(Clone, Copy, DebugBits, PartialEq, FromBits)]
pub struct DeadTime(u7);

impl Default for DeadTime {
    fn default() -> Self {
        Self {
            value: u7::new(0b010_0000),
        }
    }
}

#[bitsize(1)]
#[derive(Clone, Copy, Debug, PartialEq, Default, FromBits)]
pub enum CurrentSenseBandwidth {
    ReducedBandwidth,
    #[default]
    FullBandwidth,
}

#[bitsize(1)]
#[derive(Clone, Copy, Debug, PartialEq, Default, FromBits)]
pub enum StopOnFault {
    Disabled,
    #[default]
    Enabled,
}

#[bitsize(2)]
#[derive(Clone, Copy, Debug, PartialEq, Default, FromBits)]
pub enum DiagOutput {
    #[default]
    GeneralFault,
    Clock,
    ThresholdVoltage,
    Temperature,
}

#[bitsize(1)]
#[derive(Clone, Copy, Debug, PartialEq, Default, FromBits)]
pub enum DisableBootstrapManagement {
    #[default]
    Active,
    Disabled,
}

#[bitsize(7)]
#[derive(Clone, Copy, DebugBits, PartialEq, FromBits)]
pub struct VdsThreshold(u7);

impl Default for VdsThreshold {
    fn default() -> Self {
        Self {
            value: u7::new(0b010_0000),
        }
    }
}

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config0 {
    pub dt: DeadTime,
    pub bt: FaultBlankingTime,
}

impl AllegroRegister<u13> for Config0 {
    fn get_value(&self) -> u16 {
        self.value.into()
    }

    fn set_value(&mut self, value: u13) {
        self.value = value
    }
}

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config1 {
    pub vt: VdsThreshold,
    reserved: u1,
    pub dbm: DisableBootstrapManagement,
    pub diag: DiagOutput,
    pub esf: StopOnFault,
    pub csb: CurrentSenseBandwidth,
}

impl AllegroRegister<u13> for Config1 {
    fn get_value(&self) -> u16 {
        self.value.into()
    }

    fn set_value(&mut self, value: u13) {
        self.value = value
    }
}
