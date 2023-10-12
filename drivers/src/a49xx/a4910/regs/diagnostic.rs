//! Diagnostic register

use bilge::prelude::*;

#[bitsize(2)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct DiagnosticHeader {
    pub ff: bool,
    pub por: bool,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct DiagnosticData {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    pub vc: bool,
    pub vb: bool,
    pub va: bool,
    pub vr: bool,
    pub ot: bool,
    pub tw: bool,
}

#[bitsize(16)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct Diagnostic {
    pub data: DiagnosticData,
    reserved: u2,
    pub header: DiagnosticHeader,
}
