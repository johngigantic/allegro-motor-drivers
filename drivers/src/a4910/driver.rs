//! Main Driver for A4910

use embedded_hal::spi::SpiDevice;
use crate::error::AllegroError;

use super::{
    io::{Diagnostics, ReadRequest, ReadResponse, WriteRequest, WriteResponse},
    regs::{A4910Reg, A4910Registers},
};
pub struct A4910<SPI> {
    spi: SPI,
    pub regs: A4910Registers,
    pub status: Diagnostics,
}

impl<SPI> A4910<SPI>
where
    SPI: SpiDevice,
    AllegroError: From<SPI::Error>
{
    /// Create a driver for the motor controller connected to the SPI bus
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            regs: A4910Registers::default(),
            status: Diagnostics::default(),
        }
    }

    /// Read from the specified register, and store the register in the local data copy.
    ///
    /// # Errors
    /// Returns a `AllegroError::SpiError` if the SPI transaction fails.
    /// Stores a message but still returns a `AllegroError::MotorFault` if the fault bit is raised.
    pub fn read_register(&mut self, register: A4910Reg) -> Result<&Self, AllegroError> {
        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message)?;
        Ok(self)
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns a `AllegroError::SpiError` if the SPI transaction fails.
    /// Stores a message but still returns a `AllegroError::MotorFault` if the fault bit is raised.
    pub fn write_register(&mut self, register: A4910Reg) -> Result<&Self, AllegroError> {
        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message)?;
        Ok(self)
    }

    /// Encode a request to read the desired register.
    fn read_request(register: A4910Reg) -> [u8; 2] {
        let request: u16 = ReadRequest::new(false, register.into()).into();
        request.to_be_bytes()
    }

    /// Encode a request to write to the desired register.
    fn write_request(&self, register: A4910Reg) -> [u8; 2] {
        let reg_contents =
            unsafe { bilge::arbitrary_int::u13::new_unchecked(self.regs[register].value()) };
        let request: u16 = WriteRequest::new(reg_contents, true, register.into()).into();
        request.to_be_bytes()
    }

    /// Decode the response from a SPI read transaction.
    /// 
    /// # Errors
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn read_response(&mut self, register: A4910Reg, message: [u8; 2]) -> Result<(), AllegroError> {
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

mod tests {
    #![allow(clippy::unusual_byte_groupings)]

    #[test]
    fn test_spi_derive() {
        use super::*;
        use bilge::prelude::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected: [&Transaction; 0] = [];
        let mut a4910 = A4910::new(Mock::new(expected));

        let config0_read_req = [0b00_0_00000, 0b0000_0000];
        assert_eq!(
            A4910::<Mock>::read_request(A4910Reg::Config0),
            config0_read_req
        );

        let config1_read_req = [0b01_0_00000, 0b0000_0000];
        assert_eq!(
            A4910::<Mock>::read_request(A4910Reg::Config1),
            config1_read_req
        );

        let config1_write_req = [0b01_1_1_1_00_0, 0b0_0100000];
        assert_eq!(a4910.write_request(A4910Reg::Config1), config1_write_req);

        let config1_read_resp = [0b00_0_0_0_11_1, 0b1_1011111];
        a4910.read_response(A4910Reg::Config1, config1_read_resp).unwrap();
        assert_eq!(a4910.regs.cfg.1.vt(), u7::new(0b101_1111).into());

        a4910.spi.done();
    }
}
