//! Host-side drivers for Allegro Microsystems motor controller integrated circuits

#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]

pub mod a4910;
pub mod a4962;
pub mod a4963;
pub mod a4964;
pub mod amt49100;
pub mod amt49101;
pub mod amt49106;
pub mod amt49107;

pub mod io;
pub mod regs;
