//! SPI Message encodings for the a4910;

use bilge::prelude::*;

use super::regs::diagnostic::{Data, Header};

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadRequest {
    reserved: u13,
    write_read: bool,
    address: u2,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct ReadResponse {
    pub register: u13,
    write_read: bool,
    pub status: Header,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits)]
pub struct WriteRequest {
    register: u13,
    write_read: bool,
    address: u2,
}

#[bitsize(16)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct WriteResponse {
    pub data: Data,
    reserved: u2,
    pub header: Header,
}

pub type Diagnostics = WriteResponse;
