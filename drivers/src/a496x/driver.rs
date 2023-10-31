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
    pub fn read_register(&mut self, register: A4962Reg) -> Result<(), AllegroError> {
        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message)?;
        Ok(())
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns an `AllegroError` if the SPI transaction fails.
    pub fn write_register(&mut self, register: A4962Reg) -> Result<(), AllegroError> {
        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message)?;
        Ok(())
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
        if response.status().ff() {
            return Err(AllegroError::MotorFault);
        }
        Ok(())
    }

    /// Decode the response from a SPI write transaction.
    ///
    /// # Errors
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn write_response(&mut self, message: [u8; 2]) -> Result<(), AllegroError> {
        self.status = WriteResponse::from(u16::from_be_bytes(message));
        if self.status.header().ff() {
            return Err(AllegroError::MotorFault);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(clippy::unusual_byte_groupings)]
    #[test]
    fn test_setting_registers() {
        use super::*;
        use crate::a496x::regs::config::*;
        use bilge::prelude::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected = [
            // Read from Config1 register
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x20, 0x00], vec![0x03, 0xDF]),
            &Transaction::transaction_end(),
            // Write all Config1 register 1's to 0's, and vice versa
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0b001_1_1100, 0b0010_0000], vec![0x00, 0x00]),
            &Transaction::transaction_end(),
            // Read the switched values back from Config1
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x20, 0x00], vec![0x0C, 0x20]),
            &Transaction::transaction_end(),
        ];

        let mut a4962 = A4962::new(Mock::new(expected));

        a4962.read_register(A4962Reg::Config1).unwrap();
        assert_eq!(a4962.regs.cfg.1, Config1::default());

        a4962.regs.cfg.1.set_vt(u5::new(0b00000).into());
        a4962.regs.cfg.1.set_vdq(BemfTimeQualifier::WindowTimer);
        a4962
            .regs
            .cfg
            .1
            .set_vil(CurrentSenseThreshold::new(u4::new(0b0000)));
        a4962.regs.cfg.1.set_ipi(InvertPwmInput::InverterLogic);
        a4962.regs.cfg.1.set_pfd(PercentFastDecay::Pct25);

        a4962.write_register(A4962Reg::Config1).unwrap();
        a4962.read_register(A4962Reg::Config1).unwrap();
        assert_eq!(a4962.regs.cfg.1, u12::new(0x0C_20).into());

        a4962.spi.done();
    }

    #[test]
    fn test_transaction_errors() {
        use super::*;
        use crate::a496x::regs::config::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected = [
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x20, 0x00], vec![0x83, 0xDF]),
            &Transaction::transaction_end(),
        ];

        let mut a4962 = A4962::new(Mock::new(expected));

        let result = a4962.read_register(A4962Reg::Config1);
        assert_eq!(result, Err(AllegroError::MotorFault));
        assert_eq!(a4962.regs.cfg.1, Config1::default());

        a4962.spi.done();
    }
}
