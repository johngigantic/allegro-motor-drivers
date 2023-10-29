//! VDS Monitor registers 0-1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum SenseAmpMaximumThreshold {
    #[default]
    Mv200,
    Mv100,
    Mv50,
    Mv25,
}

#[bitsize(6)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct VdsOvervoltageThreshold(u6);

impl Default for VdsOvervoltageThreshold {
    fn default() -> Self {
        Self {
            value: u6::new(0b01_1111),
        }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum VdsFaultQualifierMode {
    #[default]
    Debounce,
    Blank,
}

#[bitsize(6)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct VdsQualifyTime(u6);

impl Default for VdsQualifyTime {
    fn default() -> Self {
        Self {
            value: u6::new(0b11_1111),
        }
    }
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct VdsMonitor0 {
    pub vt: VdsOvervoltageThreshold,
    reserved: u1,
    pub mit: SenseAmpMaximumThreshold,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct VdsMonitor1 {
    pub vqt: VdsQualifyTime,
    reserved: u1,
    pub vdq: VdsFaultQualifierMode,
    reserved: u1,
}

pub type VdsMonitor = (VdsMonitor0, VdsMonitor1);
