//! A4964 register definitions and layout

use bilge::prelude::*;
use core::ops::{Index, IndexMut};

pub mod bemf;
pub mod bridge;
pub mod commutation;
pub mod current_limit;
pub mod gate_drive;
pub mod mask;
pub mod motor_function;
pub mod nvm_write;
pub mod phase_advance;
pub mod pwm;
pub mod read_only;
pub mod readback;
pub mod speed_loop;
pub mod startup;
pub mod system;
pub mod vds_monitor;
pub mod watchdog;
pub mod write_only;

use bemf::Bemf;
use bridge::Bridge;
use commutation::Commutation;
use current_limit::CurrentLimit;
use gate_drive::GateDrive;
use mask::Mask;
use motor_function::MotorFunction;
use nvm_write::NvmWrite;
use phase_advance::PhaseAdvance;
use pwm::Pwm;
use read_only::ReadOnly;
use readback::Readback;
use speed_loop::SpeedLoop;
use startup::Startup;
use system::System;
use vds_monitor::VdsMonitor;
use watchdog::Watchdog;
use write_only::WriteOnly;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum A4964Reg {
    Pwm0,
    Pwm1,
    Bridge,
    GateDrive0,
    GateDrive1,
    GateDrive2,
    CurrentLimit,
    VdsMonitor0,
    VdsMonitor1,
    Watchdog0,
    Watchdog1,
    Commutation0,
    Commutation1,
    Bemf0,
    Bemf1,
    Startup0,
    Startup1,
    Startup2,
    Startup3,
    Startup4,
    Startup5,
    SpeedLoop0,
    SpeedLoop1,
    SpeedLoop2,
    NvmWrite,
    System,
    PhaseAdvance,
    MotorFunction,
    Mask,
    ReadbackSelect,
    WriteOnly,
    ReadOnly,
}

impl From<A4964Reg> for u8 {
    fn from(value: A4964Reg) -> Self {
        value as u8
    }
}

impl From<A4964Reg> for u5 {
    fn from(value: A4964Reg) -> Self {
        u5::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct A4964Registers {
    pub pwm: Pwm,
    pub bridge: Bridge,
    pub gate_drive: GateDrive,
    pub current_limit: CurrentLimit,
    pub vds_monitor: VdsMonitor,
    pub watchdog: Watchdog,
    pub commutation: Commutation,
    pub bemf: Bemf,
    pub startup: Startup,
    pub speed_loop: SpeedLoop,
    pub nvm_write: NvmWrite,
    pub system: System,
    pub phase_advance: PhaseAdvance,
    pub motor_function: MotorFunction,
    pub mask: Mask,
    pub readback_select: Readback,
    pub write_only: WriteOnly,
    pub read_only: ReadOnly,
}

impl Index<A4964Reg> for A4964Registers {
    type Output = dyn crate::regs::AllegroRegister;

    fn index(&self, index: A4964Reg) -> &Self::Output {
        match index {
            A4964Reg::Pwm0 => &self.pwm.0,
            A4964Reg::Pwm1 => &self.pwm.1,
            A4964Reg::Bridge => &self.bridge,
            A4964Reg::GateDrive0 => &self.gate_drive.0,
            A4964Reg::GateDrive1 => &self.gate_drive.1,
            A4964Reg::GateDrive2 => &self.gate_drive.2,
            A4964Reg::CurrentLimit => &self.current_limit,
            A4964Reg::VdsMonitor0 => &self.vds_monitor.0,
            A4964Reg::VdsMonitor1 => &self.vds_monitor.1,
            A4964Reg::Watchdog0 => &self.watchdog.0,
            A4964Reg::Watchdog1 => &self.watchdog.1,
            A4964Reg::Commutation0 => &self.commutation.0,
            A4964Reg::Commutation1 => &self.commutation.1,
            A4964Reg::Bemf0 => &self.bemf.0,
            A4964Reg::Bemf1 => &self.bemf.1,
            A4964Reg::Startup0 => &self.startup.0,
            A4964Reg::Startup1 => &self.startup.1,
            A4964Reg::Startup2 => &self.startup.2,
            A4964Reg::Startup3 => &self.startup.3,
            A4964Reg::Startup4 => &self.startup.4,
            A4964Reg::Startup5 => &self.startup.5,
            A4964Reg::SpeedLoop0 => &self.speed_loop.0,
            A4964Reg::SpeedLoop1 => &self.speed_loop.1,
            A4964Reg::SpeedLoop2 => &self.speed_loop.2,
            A4964Reg::NvmWrite => &self.nvm_write,
            A4964Reg::System => &self.system,
            A4964Reg::PhaseAdvance => &self.phase_advance,
            A4964Reg::MotorFunction => &self.motor_function,
            A4964Reg::Mask => &self.mask,
            A4964Reg::ReadbackSelect => &self.readback_select,
            A4964Reg::WriteOnly => &self.write_only,
            A4964Reg::ReadOnly => &self.read_only,
        }
    }
}

impl IndexMut<A4964Reg> for A4964Registers {
    fn index_mut(&mut self, index: A4964Reg) -> &mut Self::Output {
        match index {
            A4964Reg::Pwm0 => &mut self.pwm.0,
            A4964Reg::Pwm1 => &mut self.pwm.1,
            A4964Reg::Bridge => &mut self.bridge,
            A4964Reg::GateDrive0 => &mut self.gate_drive.0,
            A4964Reg::GateDrive1 => &mut self.gate_drive.1,
            A4964Reg::GateDrive2 => &mut self.gate_drive.2,
            A4964Reg::CurrentLimit => &mut self.current_limit,
            A4964Reg::VdsMonitor0 => &mut self.vds_monitor.0,
            A4964Reg::VdsMonitor1 => &mut self.vds_monitor.1,
            A4964Reg::Watchdog0 => &mut self.watchdog.0,
            A4964Reg::Watchdog1 => &mut self.watchdog.1,
            A4964Reg::Commutation0 => &mut self.commutation.0,
            A4964Reg::Commutation1 => &mut self.commutation.1,
            A4964Reg::Bemf0 => &mut self.bemf.0,
            A4964Reg::Bemf1 => &mut self.bemf.1,
            A4964Reg::Startup0 => &mut self.startup.0,
            A4964Reg::Startup1 => &mut self.startup.1,
            A4964Reg::Startup2 => &mut self.startup.2,
            A4964Reg::Startup3 => &mut self.startup.3,
            A4964Reg::Startup4 => &mut self.startup.4,
            A4964Reg::Startup5 => &mut self.startup.5,
            A4964Reg::SpeedLoop0 => &mut self.speed_loop.0,
            A4964Reg::SpeedLoop1 => &mut self.speed_loop.1,
            A4964Reg::SpeedLoop2 => &mut self.speed_loop.2,
            A4964Reg::NvmWrite => &mut self.nvm_write,
            A4964Reg::System => &mut self.system,
            A4964Reg::PhaseAdvance => &mut self.phase_advance,
            A4964Reg::MotorFunction => &mut self.motor_function,
            A4964Reg::Mask => &mut self.mask,
            A4964Reg::ReadbackSelect => &mut self.readback_select,
            A4964Reg::WriteOnly => &mut self.write_only,
            A4964Reg::ReadOnly => &mut self.read_only,
        }
    }
}

mod tests {
    #[test]
    fn test_default_values() {
        use super::*;

        let regs = A4964Registers::default();

        assert_eq!(regs[A4964Reg::Pwm0].get_value(), 0b0_0010_0110);
        assert_eq!(regs[A4964Reg::Pwm1].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::Bridge].get_value(), 0b0_0010_0000);
        assert_eq!(regs[A4964Reg::GateDrive0].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::GateDrive1].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::GateDrive2].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::CurrentLimit].get_value(), 0b0_0111_1111);
        assert_eq!(regs[A4964Reg::VdsMonitor0].get_value(), 0b0_0001_1111);
        assert_eq!(regs[A4964Reg::VdsMonitor1].get_value(), 0b0_0011_1111);
        assert_eq!(regs[A4964Reg::Watchdog0].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::Watchdog1].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::Commutation0].get_value(), 0b0_0111_0111);
        assert_eq!(regs[A4964Reg::Commutation1].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::Bemf0].get_value(), 0b0_0000_0100);
        assert_eq!(regs[A4964Reg::Bemf1].get_value(), 0b0_0000_0001);
        assert_eq!(regs[A4964Reg::Startup0].get_value(), 0b0_0010_0101);
        assert_eq!(regs[A4964Reg::Startup1].get_value(), 0b0_0001_1100);
        assert_eq!(regs[A4964Reg::Startup2].get_value(), 0b0_0010_0111);
        assert_eq!(regs[A4964Reg::Startup3].get_value(), 0b0_0111_0111);
        assert_eq!(regs[A4964Reg::Startup4].get_value(), 0b0_0111_0111);
        assert_eq!(regs[A4964Reg::Startup5].get_value(), 0b0_0100_0111);
        assert_eq!(regs[A4964Reg::SpeedLoop0].get_value(), 0b0_0101_0101);
        assert_eq!(regs[A4964Reg::SpeedLoop1].get_value(), 0b0_1000_0000);
        assert_eq!(regs[A4964Reg::SpeedLoop2].get_value(), 0b0_0111_0111);
        assert_eq!(regs[A4964Reg::NvmWrite].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::System].get_value(), 0b1_0100_0000);
        assert_eq!(regs[A4964Reg::PhaseAdvance].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::MotorFunction].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::Mask].get_value(), 0b1_0000_0000);
        assert_eq!(regs[A4964Reg::ReadbackSelect].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::ReadOnly].get_value(), 0b0_0000_0000);
        assert_eq!(regs[A4964Reg::WriteOnly].get_value(), 0b0_0000_0000);
    }
}
