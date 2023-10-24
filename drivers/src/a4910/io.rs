//! SPI Message encodings for the a4910;

use bilge::prelude::*;

use super::regs::diagnostic::{DiagnosticData, DiagnosticHeader};

#[bitsize(16)]
pub struct ReadRequest {
    addr: u2,
    write_read: bool,
    reserved: u13,
}

#[bitsize(16)]
pub struct ReadResponse {
    status: DiagnosticHeader,
    write_read: bool,
    register: u13,
}

#[bitsize(16)]
pub struct WriteRequest {
    addr: u2,
    write_read: bool,
    reserved: u13,
}

#[bitsize(16)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct WriteResponse {
    pub data: DiagnosticData,
    reserved: u2,
    pub header: DiagnosticHeader,
}
