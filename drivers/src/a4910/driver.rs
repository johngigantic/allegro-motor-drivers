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
        ReadRequest::new(false, reg.into()).into()
    }

    fn write_request(&self, reg: Register) -> u16 {
        WriteRequest::new(self.regs[reg].value().into(), true, reg.into()).into()
    }

    fn read_response(&mut self, reg: Register, msg: u16) {
        let r = ReadResponse::from(msg);
        self.regs[reg].set_value(r.register());
    }

    fn write_response(&mut self, msg: u16) {
        self.status = WriteResponse::from(msg);
    }
}


mod tests {
    #[test]
    fn test_spi_derive() {
        use super::*;
        use embedded_hal_mock::spi::{Mock, Transaction};

        let expected_transfers = [
            Transaction::transfer(vec![1, 2], vec![3, 4])
        ];
        let spi_device = Mock::new(&expected_transfers);
        
        let a4910 = A4910::new(spi_device);

        assert_eq!(a4910.read_request(Register::Config0), 0b00_0_0000000000000);
        assert_eq!(a4910.read_request(Register::Config1), 0b01_0_0000000000000);

        // assert_eq!(c1.write_request(), 0b01_1_1_1_00_0_0_0100000);

        // assert_eq!(c1.read_request(), 0b01_0_0000000000000);

        // c1.read_response(0b00_0_0_0_11_1_1_1011111);
        // assert_eq!(c1.vt(), u7::new(0b1011111).into());
    }

}
