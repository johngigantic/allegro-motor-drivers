//! Diagnostic register

use bilge::prelude::*;

#[bitsize(3)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct DiagnosticHeader {
    pub se: bool,
    pub por: bool,
    pub ff: bool,
}

pub type DiagnosticData = super::mask::Mask;
