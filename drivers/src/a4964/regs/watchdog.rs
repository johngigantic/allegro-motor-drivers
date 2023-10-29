//! Watchdog registers 0-1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(5)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct WatchdogMinimumTime(u5);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct WatchdogFailCount(u4);

#[bitsize(5)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct WatchdogWindowTime(u5);

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Watchdog0 {
    pub wm: WatchdogMinimumTime,
    reserved: u4,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Watchdog1 {
    pub ww: WatchdogWindowTime,
    pub wc: WatchdogFailCount,
}

pub type Watchdog = (Watchdog0, Watchdog1);
