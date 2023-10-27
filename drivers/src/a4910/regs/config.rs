//! Configuration registers 0 and 1

extern crate allegro_motor_derive;

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

use super::A4910Reg;
use crate::regs::ConstantAddress;

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
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits, AllegroRegister)]
pub struct Config0 {
    pub dt: DeadTime,
    pub bt: FaultBlankingTime,
}

impl ConstantAddress<A4910Reg> for Config0 {
    const ADDRESS: A4910Reg = A4910Reg::Config0;
}

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits, AllegroRegister)]
pub struct Config1 {
    pub vt: VdsThreshold,
    reserved: u1,
    pub dbm: DisableBootstrapManagement,
    pub diag: DiagOutput,
    pub esf: StopOnFault,
    pub csb: CurrentSenseBandwidth,
}

impl ConstantAddress<A4910Reg> for Config1 {
    const ADDRESS: A4910Reg = A4910Reg::Config1;
}

pub type Config = (Config0, Config1);
