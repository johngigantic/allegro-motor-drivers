//! Allegro A4962 and A4963 Sensorless BLDC Controller
//! 
//! The A4962 and A4963 driver and register types are identical,
//! as the two chips have the same serial interface behavior,
//! and only differ in the intended application of automotive versus consumer grade products.

pub mod driver;
pub mod io;
pub mod regs;
