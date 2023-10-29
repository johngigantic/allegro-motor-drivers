//! SPI Message encodings for the a4964 chip

use allegro_motor_derive::Parity;
use bilge::prelude::*;

use super::regs::diagnostic::{Data, Header};

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits, Parity)]
pub struct ReadRequest {
    parity: bool,
    reserved: u9,
    write_read: bool,
    address: u5,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits, Parity)]
pub struct ReadResponse {
    parity: bool,
    pub register: u9,
    write_read: bool,
    pub status: Header,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits, Parity)]
pub struct ReadOnlyResponse {
    parity: bool,
    pub register: u10,
    pub status: Header,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits, Parity)]
pub struct WriteRequest {
    parity: bool,
    register: u9,
    write_read: bool,
    address: u5,
}

#[bitsize(16)]
#[derive(DebugBits, DefaultBits, PartialEq, FromBits, Parity)]
pub struct WriteOnlyRequest {
    parity: bool,
    register: u10,
    address: u5,
}

#[bitsize(16)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits, Parity)]
pub struct WriteResponse {
    parity: bool,
    pub data: Data,
    reserved: u1,
    pub header: Header,
}

pub type Diagnostics = WriteResponse;
