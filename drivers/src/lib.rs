//! Host-side drivers for Allegro Microsystems motor controller integrated circuits

#![cfg_attr(not(test), no_std)]

pub mod a4910;
pub mod a4964;
pub mod a496x;

pub mod error;
pub mod io;
pub mod regs;
