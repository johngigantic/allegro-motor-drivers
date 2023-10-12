//! Run configuration register

use bilge::prelude::*;

#[bitsize(13)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct Run {
    pub cl: bool,
    pub ch: bool,
    pub bl: bool,
    pub bh: bool,
    pub al: bool,
    pub ah: bool,
    reserved: u7,
}
