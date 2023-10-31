//! Configuration registers 0 and 1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(6)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct FaultBlankingTime(u6);

impl Default for FaultBlankingTime {
    fn default() -> Self {
        Self {
            value: u6::new(0b10_0000),
        }
    }
}

#[bitsize(7)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct DeadTime(u7);

impl Default for DeadTime {
    fn default() -> Self {
        Self {
            value: u7::new(0b010_0000),
        }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum CurrentSenseBandwidth {
    ReducedBandwidth,
    #[default]
    FullBandwidth,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum StopOnFault {
    Disabled,
    #[default]
    Enabled,
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum DiagOutput {
    #[default]
    GeneralFault,
    Clock,
    ThresholdVoltage,
    Temperature,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum DisableBootstrapManagement {
    #[default]
    Active,
    Disabled,
}

#[bitsize(7)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct VdsThreshold(u7);

impl Default for VdsThreshold {
    fn default() -> Self {
        Self {
            value: u7::new(0b010_0000),
        }
    }
}

#[derive(AllegroRegister)]
#[bitsize(13)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Config0 {
    pub dt: DeadTime,
    pub bt: FaultBlankingTime,
}

#[derive(AllegroRegister)]
#[bitsize(13)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Config1 {
    pub vt: VdsThreshold,
    reserved: u1,
    pub dbm: DisableBootstrapManagement,
    pub diag: DiagOutput,
    pub esf: StopOnFault,
    pub csb: CurrentSenseBandwidth,
}

pub type Config = (Config0, Config1);
