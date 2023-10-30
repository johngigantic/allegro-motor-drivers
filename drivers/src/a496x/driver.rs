//! Main Driver for A4962 & A4963 Sensorless BLDC Controllers

use embedded_hal::spi::SpiDevice;

use crate::error::AllegroError;

use super::{
    io::{Diagnostics, ReadRequest, ReadResponse, WriteRequest, WriteResponse},
    regs::{A4962Reg, A4962Registers},
};

pub type A4963<SPI> = A4962<SPI>;
pub struct A4962<SPI> {
    spi: SPI,
    pub regs: A4962Registers,
    pub status: Diagnostics,
}

impl<SPI> A4962<SPI>
where
    SPI: SpiDevice,
    AllegroError: From<SPI::Error>,
{
    /// Create a driver for the motor controller connected to the SPI bus
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            regs: A4962Registers::default(),
            status: Diagnostics::default(),
        }
    }

    /// Read from the specified register, and store the register in the local data copy.
    ///
    /// # Errors
    /// Returns an `AllegroError` if the SPI transaction fails.
    pub fn read_register(&mut self, register: A4962Reg) -> Result<&Self, AllegroError> {
        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message)?;
        Ok(self)
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns an `AllegroError` if the SPI transaction fails.
    pub fn write_register(&mut self, register: A4962Reg) -> Result<&Self, AllegroError> {
        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message)?;
        Ok(self)
    }

    /// Encode a request to read the desired register.
    fn read_request(register: A4962Reg) -> [u8; 2] {
        let request: u16 = ReadRequest::new(false, register.into()).into();
        request.to_be_bytes()
    }

    /// Encode a request to write to the desired register.
    fn write_request(&self, register: A4962Reg) -> [u8; 2] {
        let reg_contents =
            unsafe { bilge::arbitrary_int::u12::new_unchecked(self.regs[register].value()) };
        let request: u16 = WriteRequest::new(reg_contents, true, register.into()).into();
        request.to_be_bytes()
    }

    /// Decode the response from a SPI read transaction.
    /// 
    /// # Errors
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn read_response(&mut self, register: A4962Reg, message: [u8; 2]) -> Result<(), AllegroError> {
        let response = ReadResponse::from(u16::from_be_bytes(message));
        self.regs[register].set_value(response.register().into());
        self.status.set_header(response.status());
        if response.status().ff() { return Err(AllegroError::MotorFault) }
        Ok(())
    }

    /// Decode the response from a SPI write transaction.
    /// 
    /// # Errors
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn write_response(&mut self, message: [u8; 2]) -> Result<(), AllegroError> {
        self.status = WriteResponse::from(u16::from_be_bytes(message));
        if self.status.header().ff() { return Err(AllegroError::MotorFault) }
        Ok(())
    }
}
