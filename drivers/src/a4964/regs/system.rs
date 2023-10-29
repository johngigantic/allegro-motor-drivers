//! System register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum StopOnFail {
    NoStop,
    #[default]
    Stop,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum LogicRegulatorVoltage {
    #[default]
    V3f3,
    V5,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum GateDriveRegulatorVoltage {
    V8,
    #[default]
    V11,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum OperatingMode {
    #[default]
    SpiOnly,
    StandAloneWithSPI,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum WakeMode {
    #[default]
    PWMWakeMode,
    LINWakeMode,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum PwmSense {
    #[default]
    ActiveHigh,
    ActiveLow,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum CurrentLimit {
    #[default]
    Enabled,
    Disabled,
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum MotorControlMode {
    #[default]
    #[fallback]
    ClosedLoopSpeed,
    ClosedLoopCurrent,
    OpenLoopSpeed,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct System {
    pub cm: MotorControlMode,
    pub dil: CurrentLimit,
    pub ipi: PwmSense,
    pub lwk: WakeMode,
    pub opm: OperatingMode,
    pub vrg: GateDriveRegulatorVoltage,
    pub vlr: LogicRegulatorVoltage,
    pub esf: StopOnFail,
}
