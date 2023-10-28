//! SPI Message encodings for the A4962 & A4963 chips.

use bilge::prelude::*;

use super::regs::diagnostic::{DiagnosticData, DiagnosticHeader};

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadRequest {
    reserved: u12,
    write_read: bool,
    address: u3,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadResponse {
    pub register: u12,
    write_read: bool,
    pub status: DiagnosticHeader,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct WriteRequest {
    register: u12,
    write_read: bool,
    address: u3,
}

#[bitsize(16)]
#[derive(PartialEq, Clone, Copy, DebugBits, Default, FromBits)]
pub struct WriteResponse {
    pub data: DiagnosticData,
    reserved: u1,
    pub header: DiagnosticHeader,
}

pub type Diagnostics = WriteResponse;
