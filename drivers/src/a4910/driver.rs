//! Main Driver for A4910

use embedded_hal::spi::SpiDevice;

use super::{
    io::{Diagnostics, ReadRequest, ReadResponse, WriteRequest, WriteResponse},
    regs::{A4910Reg, RegisterSettings},
};
pub struct A4910<SPI> {
    _spi: SPI,
    pub regs: RegisterSettings,
    pub status: Diagnostics,
}

impl<SPI> A4910<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self {
            _spi: spi,
            regs: RegisterSettings::default(),
            status: Diagnostics::default(),
        }
    }

    fn _read_request(reg: A4910Reg) -> u16 {
        ReadRequest::new(false, reg.into()).into()
    }

    fn _write_request(&self, reg: A4910Reg) -> u16 {
        WriteRequest::new(self.regs[reg].get_value().into(), true, reg.into()).into()
    }

    fn _read_response(&mut self, reg: A4910Reg, msg: u16) {
        let r = ReadResponse::from(msg);
        self.regs[reg].set_value(r.register());
    }

    fn _write_response(&mut self, msg: u16) {
        self.status = WriteResponse::from(msg);
    }
}

#[allow(clippy::unusual_byte_groupings)]
mod tests {

    #[test]
    fn test_spi_derive() {
        use super::*;
        use embedded_hal_mock::spi::Mock;

        let config0_read_req = 0b00_0_0000000000000;
        assert_eq!(
            A4910::<Mock>::_read_request(A4910Reg::Config0),
            config0_read_req
        );

        let config1_read_req = 0b01_0_0000000000000;
        assert_eq!(
            A4910::<Mock>::_read_request(A4910Reg::Config1),
            config1_read_req
        );

        // assert_eq!(c1.write_request(), 0b01_1_1_1_00_0_0_0100000);

        // assert_eq!(c1.read_request(), 0b01_0_0000000000000);

        // c1.read_response(0b00_0_0_0_11_1_1_1011111);
        // assert_eq!(c1.vt(), u7::new(0b1011111).into());
    }
}
