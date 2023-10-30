//! Main Driver for A4964

use embedded_hal::spi::SpiDevice;

use crate::io::Parity;
use crate::error::AllegroError;

use super::{
    io::{Diagnostics, ReadOnlyResponse, ReadRequest, ReadResponse, WriteRequest, WriteResponse, WriteOnlyRequest},
    regs::{A4964Reg, A4964Registers},
};
pub struct A4964<SPI> {
    spi: SPI,
    pub regs: A4964Registers,
    pub status: Diagnostics,
}

impl<SPI> A4964<SPI>
where
    SPI: SpiDevice,
    AllegroError: From<SPI::Error>,
{
    /// Create a driver for the motor controller connected to the SPI bus
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            regs: A4964Registers::default(),
            status: Diagnostics::default(),
        }
    }

    /// Read from the specified register, and store the register in the local data copy.
    ///
    /// # Errors
    /// Returns a `MotorFault` if the SPI transaction fails.
    pub fn read_register(&mut self, register: A4964Reg) -> Result<&Self, AllegroError> {
        if register == A4964Reg::WriteOnly { return Err(AllegroError::InvalidRegister) }

        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message);
        Ok(self)
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns a `MotorFault` if the SPI transaction fails.
    pub fn write_register(&mut self, register: A4964Reg) -> Result<&Self, AllegroError> {
        if register == A4964Reg::ReadOnly { return Err(AllegroError::InvalidRegister) }
    
        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message);
        Ok(self)
    }

    fn read_request(register: A4964Reg) -> [u8; 2] {
        let request: u16 = ReadRequest::new(false, false, register.into()).into();
        request.to_be_bytes()
    }

    fn write_request(&self, register: A4964Reg) -> [u8; 2] {
        let message: u16 = if register == A4964Reg::WriteOnly {
            let reg_contents = unsafe { bilge::arbitrary_int::u10::new_unchecked(self.regs[register].value()) };
            let mut request = WriteOnlyRequest::new(false, reg_contents, register.into());
            request.set_odd_parity();
            request.into()
        } else {
            let reg_contents = unsafe { bilge::arbitrary_int::u9::new_unchecked(self.regs[register].value()) };
            let mut request = WriteRequest::new(false, reg_contents, true, register.into());
            request.set_odd_parity();
            request.into()
        };
        message.to_be_bytes()
    }

    fn read_response(&mut self, register: A4964Reg, message: [u8; 2]) {
        let register_contents: u16 = if let A4964Reg::ReadOnly = register {
            let response = ReadOnlyResponse::from(u16::from_be_bytes(message));
            self.status.set_header(response.status());
            response.register().into()
        } else {
            let response = ReadResponse::from(u16::from_be_bytes(message));
            self.status.set_header(response.status());
            response.register().into()
        };
        self.regs[register].set_value(register_contents);
    }

    fn write_response(&mut self, message: [u8; 2]) {
        self.status = WriteResponse::from(u16::from_be_bytes(message));
    }
}
