//! A4962 and A4963 register definitions and layout.

use bilge::prelude::*;
use core::ops::{Index, IndexMut};

pub mod config;
pub mod diagnostic;
pub mod mask;
pub mod run;

use config::Config;
use mask::Mask;
use run::Run;

pub type A4963Reg = A4962Reg;
pub type A4963Registers = A4962Registers;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum A4962Reg {
    Config0,
    Config1,
    Config2,
    Config3,
    Config4,
    Config5,
    Mask,
    Run,
}

impl From<A4962Reg> for u8 {
    fn from(value: A4962Reg) -> Self {
        value as u8
    }
}

impl From<A4962Reg> for u3 {
    fn from(value: A4962Reg) -> Self {
        u3::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct A4962Registers {
    pub cfg: Config,
    pub mask: Mask,
    pub run: Run,
}

impl Index<A4962Reg> for A4962Registers {
    type Output = dyn crate::regs::AllegroRegister;

    fn index(&self, index: A4962Reg) -> &Self::Output {
        match index {
            A4962Reg::Config0 => &self.cfg.0,
            A4962Reg::Config1 => &self.cfg.1,
            A4962Reg::Config2 => &self.cfg.2,
            A4962Reg::Config3 => &self.cfg.3,
            A4962Reg::Config4 => &self.cfg.4,
            A4962Reg::Config5 => &self.cfg.5,
            A4962Reg::Mask => &self.mask,
            A4962Reg::Run => &self.run,
        }
    }
}

impl IndexMut<A4962Reg> for A4962Registers {
    fn index_mut(&mut self, index: A4962Reg) -> &mut Self::Output {
        match index {
            A4962Reg::Config0 => &mut self.cfg.0,
            A4962Reg::Config1 => &mut self.cfg.1,
            A4962Reg::Config2 => &mut self.cfg.2,
            A4962Reg::Config3 => &mut self.cfg.3,
            A4962Reg::Config4 => &mut self.cfg.4,
            A4962Reg::Config5 => &mut self.cfg.5,
            A4962Reg::Mask => &mut self.mask,
            A4962Reg::Run => &mut self.run,
        }
    }
}

mod tests {
    #[test]
    fn test_default_values() {
        use super::*;

        let regs = A4962Registers::default();
        assert_eq!(regs[A4962Reg::Config0].value(), 0b0010_0001_0100);
        assert_eq!(regs[A4962Reg::Config1].value(), 0b0011_1101_1111);
        assert_eq!(regs[A4962Reg::Config2].value(), 0b0111_1001_0011);
        assert_eq!(regs[A4962Reg::Config3].value(), 0b0111_0101_0010);
        assert_eq!(regs[A4962Reg::Config4].value(), 0b0111_0111_0011);
        assert_eq!(regs[A4962Reg::Config5].value(), 0b0111_0111_1000);
        assert_eq!(regs[A4962Reg::Mask].value(), 0b0000_0000_0000);
        assert_eq!(regs[A4962Reg::Run].value(), 0b0000_0000_1001);
    }
}
