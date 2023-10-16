//! Allegro A4910 Automotive 3-Phase MOSFET Driver

// use core::ops::{Index, IndexMut};

pub mod regs;

pub use regs::{
    config::{Config0, Config1},
    mask::Mask,
    run::Run,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    Config0,
    Config1,
    Mask,
    Run,
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value as u8
    }
}

pub struct RegisterSettings {
    pub cfg0: Config0,
    pub cfg1: Config1,
    pub mask: Mask,
    pub run: Run,
}

// impl Index<Register> for RegisterSettings {
//     type Output = dyn AllegroRegister;

//     fn index(&self, index: Register) -> &Self::Output {

//     }
// }

// impl IndexMut<Register> for RegisterSettings {
//     fn index_mut(&mut self, index: Register) -> &mut Self::Output {

//     }
// }
