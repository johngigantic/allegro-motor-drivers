//! Readback register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum DiagOutput {
    #[default]
    ActiveLowFaultFlag,
    FGHighWhenStationary,
    PulseHighWhenNoFault,
    PulseFGWhenNoFault,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum ResetOnSerialRead {
    #[default]
    Enabled,
    Disabled,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum LinBaudRate {
    #[default]
    Khz10,
    Khz20,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum SdoOutput {
    #[default]
    HighImpedance,
    DividedSystemClock,
}

#[bitsize(3)]
#[derive(Debug, PartialEq, Default, Copy, Clone, FromBits)]
pub enum ReadbackSelect {
    #[default]
    Diagnostic,
    MotorSpeed,
    AvgSupplyCurrent,
    SupplyVoltage,
    ChipTemperature,
    DemandInput,
    AppliedBridgePeakDutyCycle,
    AppliedPhaseAdvance,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Readback {
    pub rbs: ReadbackSelect,
    pub cks: SdoOutput,
    pub lbr: LinBaudRate,
    reserved: u1,
    pub dsr: ResetOnSerialRead,
    pub dgs: DiagOutput,
}
