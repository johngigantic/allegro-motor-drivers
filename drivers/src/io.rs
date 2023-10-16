pub trait AllegroRegister {
    fn read_request(&self) -> u16;

    fn read_response(&mut self, value: u16);

    fn write_request(&self) -> u16;
}
