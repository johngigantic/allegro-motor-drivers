//! Diagnostic register settings
//!
//! The diagnostics that can be raised are identical to the mask register.

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(5)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Header {
    pub cli: bool,
    pub vpu: bool,
    pub se: bool,
    pub por: bool,
    pub ff: bool,
}

/// Diagnostic faults
///
/// The diagnostic data here is nearly identical to the mask data,
/// except with a default value of 0 for the watchdog bit.
#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits, DefaultBits)]
pub struct Data {
    pub vo: bool,
    pub bu: bool,
    pub vlu: bool,
    pub vru: bool,
    pub vsu: bool,
    pub tw: bool,
    pub ot: bool,
    pub los: bool,
    pub wd: bool,
}
