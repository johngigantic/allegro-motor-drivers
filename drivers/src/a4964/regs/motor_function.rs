//! Motor function register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum LinState {
    #[default]
    Standby,
    Active,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum SleepTransition {
    #[default]
    NoChange,
    EnterSleepStateIfEnabled,
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum Overmodulation {
    #[default]
    None,
    Modulation112f5,
    Modulation125,
    Modulation150,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum DriveMode {
    #[default]
    Sinusoidal,
    Trapezoidal,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum BrakeMode {
    #[default]
    BrakeDisabled,
    BrakeEnabled,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum Direction {
    #[default]
    Forward,
    Reverse,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum MotorRunningStatus {
    #[default]
    Disable,
    Run,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct MotorFunction {
    reserved: u1,
    pub len: LinState,
    pub gts: SleepTransition,
    pub ovm: Overmodulation,
    pub drm: DriveMode,
    pub brk: BrakeMode,
    pub dir: Direction,
    pub run: MotorRunningStatus,
}
