//! Mask register contents

use bilge::prelude::*;

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct Mask {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u1,
    pub vs: bool,
    reserved: u1,
    pub los: bool,
    pub ot: bool,
    pub tw: bool,
}
