//! Phase Advance register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(1)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum PhaseAdvanceMode {
    #[default]
    Manual,
    Automatic,
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Copy, Clone, Default, FromBits)]
pub enum PhaseAdvanceGain {
    #[default]
    Gain1,
    Gain2,
    Gain4,
    Gain8,
}

#[bitsize(6)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct PhaseAdvanceValue(u6);

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct PhaseAdvance {
    pub pa: PhaseAdvanceValue,
    pub kip: PhaseAdvanceGain,
    pub pam: PhaseAdvanceMode,
}
