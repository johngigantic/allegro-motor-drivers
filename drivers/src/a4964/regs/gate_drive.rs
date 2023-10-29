//! Gate Drive registers 0-2

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TurnOnCurrent1(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TurnOnCurrent2(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TurnOffCurrent1(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct TurnOffCurrent2(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct SlewControlOnTime(u4);

#[bitsize(4)]
#[derive(DebugBits, PartialEq, DefaultBits, FromBits)]
pub struct SlewControlOffTime(u4);

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct GateDrive0 {
    pub ir2: TurnOnCurrent2,
    pub ir1: TurnOnCurrent1,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct GateDrive1 {
    pub if2: TurnOffCurrent2,
    pub if1: TurnOffCurrent1,
    reserved: u1,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct GateDrive2 {
    pub tfs: SlewControlOffTime,
    pub trs: SlewControlOnTime,
    reserved: u1,
}

pub type GateDrive = (GateDrive0, GateDrive1, GateDrive2);
