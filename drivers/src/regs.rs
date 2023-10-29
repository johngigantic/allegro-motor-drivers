//! `RegisterSettings` and Register abstractions

use core::ops::IndexMut;

pub trait RegisterSettings<Register>: IndexMut<Register> {}

pub trait AllegroRegister {
    fn get_value(&self) -> u16;
    fn set_value(&mut self, value: u16);
}
