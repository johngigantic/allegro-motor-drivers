//! SPI Message encodings for the A4962 & A4963 chips.

use bilge::prelude::*;

use super::regs::diagnostic::{Data, Header};

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
    pub status: Header,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct WriteRequest {
    register: u12,
    write_read: bool,
    address: u3,
}

#[bitsize(16)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct WriteResponse {
    pub data: Data,
    reserved: u1,
    pub header: Header,
}

pub type Diagnostics = WriteResponse;
