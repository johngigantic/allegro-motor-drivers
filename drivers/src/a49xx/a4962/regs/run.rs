//! Run register

use bilge::prelude::*;

#[bitsize(2)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum MotorControlMode {
    #[default]
    IndirectSpeed,
    DirectSpeed,
    ClosedLoopCurrent,
    ClosedLoopSpeed,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum StopOnFail {
    #[default]
    NoStop,
    Stop,
}

#[bitsize(5)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct DutyCycleControl(u5);

impl Default for DutyCycleControl {
    fn default() -> Self {
        Self { value: u5::new(0b0_0000), }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum RestartControl {
    NoRestart,
    #[default]
    AllowRestart,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum Brake {
    #[default]
    Disable,
    Start,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum Direction {
    #[default]
    Forward,
    Reverse,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum RunEnable {
    Disable,
    #[default]
    Start,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Run {
    pub run: RunEnable,
    pub dir: Direction,
    pub brk: Brake,
    pub rsc: RestartControl,
    pub di: DutyCycleControl,
    pub esf: StopOnFail,
    pub cm: MotorControlMode,
}

