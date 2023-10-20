#![cfg(test)]

/// TODO(John Little): Comment in the failing example tests when the code is complete.
#[test]
fn test_spi_derive() {
    use allegro_motor_drivers::a49xx::a4910::regs::config::*;
    use allegro_motor_drivers::io::SpiMessages;
    use bilge::prelude::*;
    
    let mut c1 = Config1::default();
    assert_eq!(c1.write_request(), 0b01_1_1_1_00_0_0_0100000);
    
    assert_eq!(c1.read_request(), 0b01_0_0000000000000);
    
    c1.read_response(0b00_0_0_0_11_1_1_1011111);
    assert_eq!(c1.vt(), u7::new(0b1011111).into());
    
}
