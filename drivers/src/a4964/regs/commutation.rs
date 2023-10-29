//! Commutation registers 0-1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct SteadyStateProportionalGain(u4);

impl Default for SteadyStateProportionalGain {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct SteadyStateIntegralGain(u4);

impl Default for SteadyStateIntegralGain {
    fn default() -> Self {
        Self {
            value: u4::new(0b0111),
        }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TransientProportionalGain(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TransientIntegralGain(u4);

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Commutation0 {
    pub ci: SteadyStateIntegralGain,
    pub cp: SteadyStateProportionalGain,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Commutation1 {
    pub cit: TransientIntegralGain,
    pub cpt: TransientProportionalGain,
    reserved: u1,
}

pub type Commutation = (Commutation0, Commutation1);
