//! Main Driver for A4910

use embedded_hal::spi::SpiDevice;

use super::{
    io::{Diagnostics, ReadRequest, ReadResponse, WriteRequest, WriteResponse},
    regs::{Register, RegisterSettings},
};
pub struct A4910<SPI> {
    spi: SPI,
    regs: RegisterSettings,
    status: Diagnostics,
}

impl<SPI> A4910<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            regs: RegisterSettings::default(),
            status: Diagnostics::default(),
        }
    }

    fn read_request(&self, reg: Register) -> u16 {
        ReadRequest::new(reg.into(), false).into()
    }

    fn write_request(&self, reg: Register) -> u16 {
        WriteRequest::new(reg.into(), true, self.regs[reg].value().into()).into()
    }

    fn read_response(&mut self, reg: Register, msg: u16) {
        let r = ReadResponse::from(msg);
        self.regs[reg].set_value(r.register());
    }

    fn write_response(&mut self, msg: u16) {
        self.status = WriteResponse::from(msg);
    }
}
