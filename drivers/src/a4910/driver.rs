//! Main Driver for A4910

use crate::error::AllegroError;
use embedded_hal::spi::SpiDevice;

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
    AllegroError: From<SPI::Error>,
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
    pub fn read_register(&mut self, register: A4910Reg) -> Result<(), AllegroError> {
        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message)?;
        Ok(())
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns a `AllegroError::SpiError` if the SPI transaction fails.
    /// Stores a message but still returns a `AllegroError::MotorFault` if the fault bit is raised.
    pub fn write_register(&mut self, register: A4910Reg) -> Result<(), AllegroError> {
        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message)?;
        Ok(())
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
        use crate::a4910::regs::config::*;
        use bilge::prelude::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected = [
            // Read from Config1 register
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x40, 0x00], vec![0x18, 0x20]),
            &Transaction::transaction_end(),
            // Write all Config1 register 1's to 0's, and vice versa
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0b01_1_00111, 0b0101_1111], vec![0x00, 0x00]),
            &Transaction::transaction_end(),
            // Read the switched values back from Config1
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x40, 0x00], vec![0x1F, 0x5F]),
            &Transaction::transaction_end(),
        ];

        let mut a4910 = A4910::new(Mock::new(expected));

        a4910.read_register(A4910Reg::Config1).unwrap();
        assert_eq!(a4910.regs.cfg.1, Config1::default());

        a4910.regs.cfg.1.set_vt(u7::new(0b101_1111).into());
        a4910
            .regs
            .cfg
            .1
            .set_dbm(DisableBootstrapManagement::Disabled);
        a4910.regs.cfg.1.set_diag(DiagOutput::Temperature);
        a4910.regs.cfg.1.set_esf(StopOnFault::Disabled);
        a4910
            .regs
            .cfg
            .1
            .set_csb(CurrentSenseBandwidth::ReducedBandwidth);

        a4910.write_register(A4910Reg::Config1).unwrap();
        a4910.read_register(A4910Reg::Config1).unwrap();
        assert_eq!(a4910.regs.cfg.1, u13::new(0x1F_5F).into());

        a4910.spi.done();
    }

    #[test]
    fn test_transaction_errors() {
        use super::*;
        use crate::a4910::regs::config::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected = [
            &Transaction::transaction_start(),
            &Transaction::transfer_in_place(vec![0x40, 0x00], vec![0x98, 0x20]),
            &Transaction::transaction_end(),
        ];

        let mut a4910 = A4910::new(Mock::new(expected));

        let result = a4910.read_register(A4910Reg::Config1);
        assert_eq!(result, Err(AllegroError::MotorFault));
        assert_eq!(a4910.regs.cfg.1, Config1::default());

        a4910.spi.done();
    }
}
