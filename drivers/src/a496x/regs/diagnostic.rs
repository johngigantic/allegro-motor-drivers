//! Diagnostic register settings
//!
//! The diagnostics that can be raised are identical to the mask register.

use bilge::prelude::*;

#[bitsize(3)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Header {
    pub se: bool,
    pub por: bool,
    pub ff: bool,
}

pub type Data = super::mask::Mask;
