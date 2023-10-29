//! Diagnostic register

use bilge::prelude::*;

#[bitsize(2)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Header {
    pub por: bool,
    pub ff: bool,
}

#[bitsize(12)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Data {
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
