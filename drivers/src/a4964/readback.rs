//! Readback telemetry

use core::ops::{Index, IndexMut};
use bilge::prelude::*;
use super::regs::readback::ReadbackSelect;

#[bitsize(10)]
#[derive(Copy, Clone, PartialEq, DebugBits, DefaultBits, FromBits)]
pub struct ReadbackDiagnostics {
    cl: bool,
    ch: bool,
    bl: bool,
    bh: bool,
    al: bool,
    ah: bool,
    bc: bool,
    bb: bool,
    ba: bool,
    osr: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Readback {
    diagnostic: u16,
    motor_speed: u16,
    avg_supply_current: u16,
    supply_voltage: u16,
    chip_temperature: u16,
    demand_input: u16,
    applied_bridge_peak_duty_cycle: u16,
    applied_phase_advance: u16,
}

impl Index<ReadbackSelect> for Readback {
    type Output = u16;
    fn index(&self, index: ReadbackSelect) -> &Self::Output {
        match index {
            ReadbackSelect::Diagnostic => &self.diagnostic,
            ReadbackSelect::MotorSpeed => &self.motor_speed,
            ReadbackSelect::AvgSupplyCurrent => &self.avg_supply_current,
            ReadbackSelect::SupplyVoltage => &self.supply_voltage,
            ReadbackSelect::ChipTemperature => &self.chip_temperature,
            ReadbackSelect::DemandInput => &self.demand_input,
            ReadbackSelect::AppliedBridgePeakDutyCycle => &self.applied_bridge_peak_duty_cycle,
            ReadbackSelect::AppliedPhaseAdvance => &self.applied_phase_advance,
        }
    }
}

impl IndexMut<ReadbackSelect> for Readback {
    fn index_mut(&mut self, index: ReadbackSelect) -> &mut Self::Output {
        match index {
            ReadbackSelect::Diagnostic => &mut self.diagnostic,
            ReadbackSelect::MotorSpeed => &mut self.motor_speed,
            ReadbackSelect::AvgSupplyCurrent => &mut self.avg_supply_current,
            ReadbackSelect::SupplyVoltage => &mut self.supply_voltage,
            ReadbackSelect::ChipTemperature => &mut self.chip_temperature,
            ReadbackSelect::DemandInput => &mut self.demand_input,
            ReadbackSelect::AppliedBridgePeakDutyCycle => &mut self.applied_bridge_peak_duty_cycle,
            ReadbackSelect::AppliedPhaseAdvance => &mut self.applied_phase_advance,
        }
    }
}