//! RegisterSettings and Register abstractions

use core::ops::IndexMut;

pub trait RegisterSettings<Register>: IndexMut<Register> {}

pub trait ConstantAddress<Register> {
    const ADDRESS: Register;
}
