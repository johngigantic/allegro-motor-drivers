//! `RegisterSettings` and Register abstractions

use core::ops::IndexMut;

pub trait RegisterSettings<Register>: IndexMut<Register> {}

pub trait AllegroRegister {
    fn value(&self) -> u16;
    fn set_value(&mut self, value: u16);
}
