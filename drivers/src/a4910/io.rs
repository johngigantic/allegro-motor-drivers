//! SPI Message encodings for the a4910;

use bilge::prelude::*;

use super::regs::diagnostic::{DiagnosticData, DiagnosticHeader};

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadRequest {
    reserved: u13,
    write_read: bool,
    addr: u2,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadResponse {
    pub register: u13,
    write_read: bool,
    pub status: DiagnosticHeader,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct WriteRequest {
    register: u13,
    write_read: bool,
    addr: u2,
}

#[bitsize(16)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct WriteResponse {
    pub data: DiagnosticData,
    reserved: u2,
    pub header: DiagnosticHeader,
}

pub type Diagnostics = WriteResponse;
