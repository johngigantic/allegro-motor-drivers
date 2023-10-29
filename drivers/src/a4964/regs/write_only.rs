//! Write Only register

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[derive(AllegroRegister)]
#[bitsize(10)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct WriteOnly {
    pub demand_input: u10,
}
