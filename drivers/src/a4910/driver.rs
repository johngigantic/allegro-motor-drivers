//! Main Driver for A4910

use embedded_hal::spi::SpiDevice;

use super::{
    io::{Diagnostics, ReadRequest, ReadResponse, WriteRequest, WriteResponse},
    regs::{A4910Reg, RegisterSettings},
};
pub struct A4910<SPI> {
    _spi: SPI,
    _regs: RegisterSettings,
    _status: Diagnostics,
}

impl<SPI> A4910<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self {
            _spi: spi,
            _regs: RegisterSettings::default(),
            _status: Diagnostics::default(),
        }
    }

    fn _read_request(&self, reg: A4910Reg) -> u16 {
        ReadRequest::new(false, reg.into()).into()
    }

    fn _write_request(&self, reg: A4910Reg) -> u16 {
        WriteRequest::new(self._regs[reg].get_value().into(), true, reg.into()).into()
    }

    fn _read_response(&mut self, reg: A4910Reg, msg: u16) {
        let r = ReadResponse::from(msg);
        self._regs[reg].set_value(r.register());
    }

    fn _write_response(&mut self, msg: u16) {
        self._status = WriteResponse::from(msg);
    }
}

mod tests {
    #[test]
    fn test_spi_derive() {
        use super::*;
        use embedded_hal_mock::spi::{Mock, Transaction};

        let expected_transfers = [Transaction::transfer(vec![1, 2], vec![3, 4])];
        let spi_device = Mock::new(&expected_transfers);

        let a4910 = A4910::new(spi_device);

        assert_eq!(a4910._read_request(A4910Reg::Config0), 0b00_0_0000000000000);
        assert_eq!(a4910._read_request(A4910Reg::Config1), 0b01_0_0000000000000);

        // assert_eq!(c1.write_request(), 0b01_1_1_1_00_0_0_0100000);

        // assert_eq!(c1.read_request(), 0b01_0_0000000000000);

        // c1.read_response(0b00_0_0_0_11_1_1_1011111);
        // assert_eq!(c1.vt(), u7::new(0b1011111).into());
    }
}
