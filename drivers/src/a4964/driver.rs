//! Main Driver for A4964

use embedded_hal::spi::SpiDevice;

use crate::error::AllegroError;
use crate::io::Parity;

use super::{
    diagnostics::Diagnostics,
    io::*,
    readback::Readback,
    regs::{readback::ReadbackSelect, *},
};
pub struct A4964<SPI> {
    spi: SPI,
    pub regs: A4964Registers,
    pub diagnostics: Diagnostics,
    pub readback: Readback,
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
            diagnostics: Diagnostics::default(),
            readback: Readback::default(),
        }
    }

    /// Read from the specified register, and store the register in the local data copy.
    ///
    /// # Errors
    /// Returns an `AllegroError` if the SPI transaction fails.
    /// Returns an `InvalidRegister` if the user is trying to read from the write only register
    pub fn read_register(&mut self, register: A4964Reg) -> Result<(), AllegroError> {
        if register == A4964Reg::WriteOnly {
            return Err(AllegroError::InvalidRegister);
        }

        let mut message = Self::read_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.read_response(register, message)?;
        Ok(())
    }

    /// Write to the specified register, and store the returned diagnostics.
    ///
    /// # Errors
    /// Returns a `MotorFault` if the SPI transaction fails.
    pub fn write_register(&mut self, register: A4964Reg) -> Result<(), AllegroError> {
        if register == A4964Reg::ReadOnly {
            return Err(AllegroError::InvalidRegister);
        }

        let mut message = self.write_request(register);
        self.spi.transfer_in_place(&mut message)?;
        self.write_response(message)?;
        Ok(())
    }

    /// Encode a request to read the desired register.
    fn read_request(register: A4964Reg) -> [u8; 2] {
        let request: u16 = ReadRequest::new(false, false, register.into()).into();
        request.to_be_bytes()
    }

    /// Encode a request to write to the desired register.
    fn write_request(&self, register: A4964Reg) -> [u8; 2] {
        let message: u16 = if register == A4964Reg::WriteOnly {
            let reg_contents =
                unsafe { bilge::arbitrary_int::u10::new_unchecked(self.regs[register].value()) };
            let mut request = WriteOnlyRequest::new(false, reg_contents, register.into());
            request.set_odd_parity();
            request.into()
        } else {
            let reg_contents =
                unsafe { bilge::arbitrary_int::u9::new_unchecked(self.regs[register].value()) };
            let mut request = WriteRequest::new(false, reg_contents, true, register.into());
            request.set_odd_parity();
            request.into()
        };
        message.to_be_bytes()
    }

    /// Decode the response from a SPI read transaction.
    ///
    /// # Errors
    /// If the message has `AllegroError::InvalidParity`, then it is not to be trusted.
    /// The message is not parsed, and the function returns.
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn read_response(&mut self, register: A4964Reg, message: [u8; 2]) -> Result<(), AllegroError> {
        let mut result: Result<(), AllegroError> = Ok(());
        let message = u16::from_be_bytes(message);
        if !crate::io::parity(message) {
            return Err(AllegroError::InvalidParity);
        }

        let register_contents: u16 = if register == A4964Reg::ReadOnly {
            let response = ReadOnlyResponse::from(message);
            if response.status().ff() {
                result = Err(AllegroError::MotorFault);
            }
            self.diagnostics.set_main_diagnostics(response.status());

            let register_contents: u16 = response.register().into();
            self.readback[self.regs.readback_select.rbs()] = register_contents;

            if self.regs.readback_select.rbs() == ReadbackSelect::Diagnostic {
                self.diagnostics
                    .set_readback_diagnostics(response.register().into());
            }

            register_contents
        } else {
            let response = ReadResponse::from(message);
            if response.status().ff() {
                result = Err(AllegroError::MotorFault);
            }

            self.diagnostics.set_main_diagnostics(response.status());
            response.register().into()
        };
        self.regs[register].set_value(register_contents);
        result
    }

    /// Decode the response from a SPI write transaction.
    ///
    /// # Errors
    /// If the message has `AllegroError::InvalidParity`, then it is not to be trusted.
    /// The message is not parsed, and the function returns.
    /// This function will check for an `AllegroError::MotorFault` and return it
    /// after parsing.
    fn write_response(&mut self, message: [u8; 2]) -> Result<(), AllegroError> {
        let message = u16::from_be_bytes(message);
        if !crate::io::parity(message) {
            return Err(AllegroError::InvalidParity);
        }
        let resp = WriteResponse::from(message);
        self.diagnostics.set_main_diagnostics(resp.header());
        self.diagnostics.set_status_diagnostics(resp.data());
        if resp.header().ff() {
            return Err(AllegroError::MotorFault);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn a4964() {
        use super::*;
        use embedded_hal_mock::eh1::spi::{Mock, Transaction};

        let expected: [&Transaction; 0] = [];
        let mut a4964 = A4964::new(Mock::new(expected));

        let a = 1;
        let b = 1;

        assert_eq!(a, b);

        a4964.spi.done();
    }
}
