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
pub enum A4910Reg {
    Config0,
    Config1,
    Mask,
    Run,
}

impl From<A4910Reg> for u8 {
    fn from(value: A4910Reg) -> Self {
        value as u8
    }
}

impl From<A4910Reg> for u2 {
    fn from(value: A4910Reg) -> Self {
        u2::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct RegisterSettings {
    pub cfg: Config,
    pub mask: Mask,
    pub run: Run,
}

impl Index<A4910Reg> for RegisterSettings {
    type Output = dyn crate::regs::AllegroRegister<u13>;

    fn index(&self, index: A4910Reg) -> &Self::Output {
        match index {
            A4910Reg::Config0 => &self.cfg.0,
            A4910Reg::Config1 => &self.cfg.1,
            A4910Reg::Mask => &self.mask,
            A4910Reg::Run => &self.run,
        }
    }
}

impl IndexMut<A4910Reg> for RegisterSettings {
    fn index_mut(&mut self, index: A4910Reg) -> &mut Self::Output {
        match index {
            A4910Reg::Config0 => &mut self.cfg.0,
            A4910Reg::Config1 => &mut self.cfg.1,
            A4910Reg::Mask => &mut self.mask,
            A4910Reg::Run => &mut self.run,
        }
    }
}

impl crate::regs::RegisterSettings<A4910Reg> for RegisterSettings {}
