use bilge::prelude::*;
use core::ops::{Index, IndexMut};

pub mod config;
pub mod diagnostic;
pub mod dumb;
pub mod mask;
pub mod run;

use config::{Config0, Config1};
use mask::Mask;
use run::Run;

pub trait AllegroRegister {
    fn value(&self) -> u16;
    fn set_value(&mut self, value: u13);
}

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

impl From<Register> for u2 {
    fn from(value: Register) -> Self {
        u2::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct RegisterSettings {
    pub cfg0: Config0,
    pub cfg1: Config1,
    pub mask: Mask,
    pub run: Run,
}

impl Index<Register> for RegisterSettings {
    type Output = dyn AllegroRegister;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::Config0 => &self.cfg0,
            Register::Config1 => &self.cfg1,
            Register::Mask => &self.mask,
            Register::Run => &self.run,
        }
    }
}

impl IndexMut<Register> for RegisterSettings {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::Config0 => &mut self.cfg0,
            Register::Config1 => &mut self.cfg1,
            Register::Mask => &mut self.mask,
            Register::Run => &mut self.run,
        }
    }
}
