//! Input/output messaging between hosts and peripherals.
use bilge::prelude::*;

#[bitsize(16)]
pub struct ReadRequest {
    addr: u5,
    write_read: bool,
    register: u9,
    parity: bool,
}

#[bitsize(16)]
pub struct ReadResponse {
    parity: bool,
    pub register: u9,
    pub write_read: bool,
    pub status_header: u5,
}

#[bitsize(16)]
pub struct WriteRequest {
    addr: u5,
    write_read: bool,
    register: u9,
    parity: bool,
}
