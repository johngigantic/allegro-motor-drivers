//! All diagnostics for the chip

use bilge::prelude::*;
use crate::a4964::readback::ReadbackDiagnostics;
use crate::a4964::regs::diagnostic::*;

#[bitsize(24)]
#[derive(Clone, Copy, DefaultBits, DebugBits, FromBits)]
pub struct Diagnostics {
    pub readback_diagnostics: ReadbackDiagnostics,
    pub status_diagnostics: Data,
    pub main_diagnostics: Header,
}
