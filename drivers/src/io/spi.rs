//! Input/output messaging between hosts and peripherals.
use bilge::prelude::*;

/// A trait abstracting SPI Messages to and from the motor driver.
///
/// Implements read_request, read_response, and write_request functions
/// to encode and decode u16 messages into and out of a chip's register.
///
/// Note: the write_response function is separately defined separately for
/// each chip's protocol. Write responses contain a copy of the diagnostic
/// register, so the written register has no bearing on the output message.
pub trait Messages {
    fn read_request(&self) -> u16;

    fn read_response(&mut self, value: u16);

    fn write_request(&self) -> u16;
}

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
