//! Back EMF registers 0-1

use allegro_motor_derive::AllegroRegister;
use bilge::prelude::*;

#[bitsize(5)]
#[derive(PartialEq, Copy, Clone, DebugBits, FromBits)]
pub struct DetectionWindow(u5);

impl Default for DetectionWindow {
    fn default() -> Self {
        Self {
            value: u5::new(0b00100),
        }
    }
}

#[bitsize(2)]
#[derive(Copy, Clone, Debug, PartialEq, Default, FromBits)]
pub enum SamplesPerCycle {
    #[default]
    Samples1,
    Samples2,
    Samples3,
    Samples6,
}

#[bitsize(4)]
#[derive(Copy, Clone, Debug, PartialEq, Default, FromBits)]
pub enum WindmillBemfFilterTime {
    Time0,
    #[default]
    Time200us,
    Time400us,
    Time600us,
    Time800us,
    Time1ms,
    Time2ms,
    Time4ms,
    Time5ms,
    Time6ms,
    Time10ms,
    Time12ms,
    Time14ms,
    Time16ms,
    Time18ms,
    Time20ms,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Bemf0 {
    pub bw: DetectionWindow,
    reserved: u4,
}

#[derive(AllegroRegister)]
#[bitsize(9)]
#[derive(PartialEq, Copy, Clone, DebugBits, DefaultBits, FromBits)]
pub struct Bemf1 {
    pub bf: WindmillBemfFilterTime,
    pub bs: SamplesPerCycle,
    reserved: u3,
}

pub type Bemf = (Bemf0, Bemf1);
