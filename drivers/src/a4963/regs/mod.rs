use bilge::prelude::*;
use core::ops::{Index, IndexMut};

pub mod config;
pub mod diagnostic;
pub mod mask;
pub mod run;

use config::Config;
use mask::Mask;
use run::Run;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum A4963Reg {
    Config0,
    Config1,
    Config2,
    Config3,
    Config4,
    Config5,
    Mask,
    Run,
}

impl From<A4963Reg> for u8 {
    fn from(value: A4963Reg) -> Self {
        value as u8
    }
}

impl From<A4963Reg> for u3 {
    fn from(value: A4963Reg) -> Self {
        u3::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct A4963Registers {
    pub cfg: Config,
    pub mask: Mask,
    pub run: Run,
}

impl Index<A4963Reg> for A4963Registers {
    type Output = dyn crate::regs::AllegroRegister<u12>;

    fn index(&self, index: A4963Reg) -> &Self::Output {
        match index {
            A4963Reg::Config0 => &self.cfg.0,
            A4963Reg::Config1 => &self.cfg.1,
            A4963Reg::Config2 => &self.cfg.2,
            A4963Reg::Config3 => &self.cfg.3,
            A4963Reg::Config4 => &self.cfg.4,
            A4963Reg::Config5 => &self.cfg.5,
            A4963Reg::Mask => &self.mask,
            A4963Reg::Run => &self.run,
        }
    }
}

impl IndexMut<A4963Reg> for A4963Registers {
    fn index_mut(&mut self, index: A4963Reg) -> &mut Self::Output {
        match index {
            A4963Reg::Config0 => &mut self.cfg.0,
            A4963Reg::Config1 => &mut self.cfg.1,
            A4963Reg::Config2 => &mut self.cfg.2,
            A4963Reg::Config3 => &mut self.cfg.3,
            A4963Reg::Config4 => &mut self.cfg.4,
            A4963Reg::Config5 => &mut self.cfg.5,
            A4963Reg::Mask => &mut self.mask,
            A4963Reg::Run => &mut self.run,
        }
    }
}

mod tests {
    #[test]
    fn test_default_values() {
        use super::*;

        let regs = A4963Registers::default();
        assert_eq!(regs[A4963Reg::Config0].get_value(), 0b0000_0010_0001_0100);
        assert_eq!(regs[A4963Reg::Config1].get_value(), 0b0000_0011_1101_1111);
        assert_eq!(regs[A4963Reg::Config2].get_value(), 0b0000_0111_1001_0011);
        assert_eq!(regs[A4963Reg::Config3].get_value(), 0b0000_0111_0101_0010);
        assert_eq!(regs[A4963Reg::Config4].get_value(), 0b0000_0111_0111_0011);
        assert_eq!(regs[A4963Reg::Config5].get_value(), 0b0000_0111_0111_1000);
        assert_eq!(regs[A4963Reg::Mask].get_value(), 0b0000_0000_0000_0000);
        assert_eq!(regs[A4963Reg::Run].get_value(), 0b0000_0000_0000_1001);
    }
}
