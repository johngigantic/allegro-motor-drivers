//! RegisterSettings and Register abstractions

use core::ops::IndexMut;

pub trait RegisterSettings<Register>: IndexMut<Register> {}

pub trait ConstantAddress<Register> {
    const ADDRESS: Register;
}

pub trait AllegroRegister<Number> {
    fn get_value(&self) -> u16;
    fn set_value(&mut self, value: Number);
}
