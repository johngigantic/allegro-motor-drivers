//! A4962 configuration registers

use bilge::prelude::*;

#[bitsize(2)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum RecirculationMode {
    #[default]
    Auto,
    High,
    Low,
    Off,
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct BlankTime(u4);

impl Default for BlankTime {
    fn default() -> Self {
        Self { value: u4::new(0b1000), }
    }
}

#[bitsize(6)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct DeadTime(u6);

impl Default for DeadTime {
    fn default() -> Self {
        Self { value: u6::new(0b01_0100), }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum PercentFastDecay {
    #[default]
    Pct12F5,
    Pct25,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum InvertPwmInput {
    #[default]
    NormalTrueLogic,
    InverterLogic,
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct CurrentSenseThreshold(u4);

impl Default for CurrentSenseThreshold {
    fn default() -> Self {
        Self { value: u4::new(0b1111), }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum BemfTimeQualifier {
    #[default]
    DebounceTimer,
    WindowTimer,
}

#[bitsize(5)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct VdsThreshold(u5);

impl Default for VdsThreshold {
    fn default() -> Self {
        Self { value: u5::new(0b1_1111), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct PositionProportionalGain(u4);

impl Default for PositionProportionalGain {
    fn default() -> Self {
        Self { value: u4::new(0b0111), }
    }
}

#[bitsize(2)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum OverspeedLimitRatio {
    Pct100,
    Pct125,
    #[default]
    Pct150,
    Pct200,
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum DegaussCompensation {
    #[default]
    Off,
    Active,
}

#[bitsize(5)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct FixedOffTime(u5);

impl Default for FixedOffTime {
    fn default() -> Self {
        Self { value: u5::new(0b1_0011), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct PositionIntegralGain(u4);

impl Default for PositionIntegralGain {
    fn default() -> Self {
        Self { value: u4::new(0b0111), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct HoldPwmDutyCycle(u4);

impl Default for HoldPwmDutyCycle {
    fn default() -> Self {
        Self { value: u4::new(0b0101), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct HoldTime(u4);

impl Default for HoldTime {
    fn default() -> Self {
        Self { value: u4::new(0b0010), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct SpeedProportionalGain(u4);

impl Default for SpeedProportionalGain {
    fn default() -> Self {
        Self { value: u4::new(0b0111), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct ForcedPwmDutyCycle(u4);

impl Default for ForcedPwmDutyCycle {
    fn default() -> Self {
        Self { value: u4::new(0b0111), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct StartSpeed(u4);

impl Default for StartSpeed {
    fn default() -> Self {
        Self { value: u4::new(0b0011), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct SpeedIntegralGain(u4);

impl Default for SpeedIntegralGain {
    fn default() -> Self {
        Self { value: u4::new(0b0111), }
    }
}

#[bitsize(1)]
#[derive(Debug, PartialEq, Default, FromBits)]
pub enum SpeedOutputSelection {
    #[default]
    FG,
    TACHO,
}

#[bitsize(3)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct MaximumSpeed(u3);

impl Default for MaximumSpeed {
    fn default() -> Self {
        Self { value: u3::new(0b101), }
    }
}

#[bitsize(4)]
#[derive(DebugBits, PartialEq, FromBits)]
pub struct PhaseAdvance(u4);

impl Default for PhaseAdvance {
    fn default() -> Self {
        Self { value: u4::new(0b1000), }
    }
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config0 {
    pub dt: DeadTime,
    pub bt: BlankTime,
    pub rm: RecirculationMode,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config1 {
    pub vt: VdsThreshold,
    pub vdq: BemfTimeQualifier,
    pub vil: CurrentSenseThreshold,
    pub ipi: InvertPwmInput,
    pub pfd: PercentFastDecay,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config2 {
    pub pw: FixedOffTime,
    pub dgc: DegaussCompensation,
    pub sh: OverspeedLimitRatio,
    pub cp: PositionProportionalGain,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config3 {
    pub ht: HoldTime,
    pub hd: HoldPwmDutyCycle,
    pub ci: PositionIntegralGain,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config4 {
    pub ss: StartSpeed,
    pub sd: ForcedPwmDutyCycle,
    pub sp: SpeedProportionalGain,
}

#[bitsize(12)]
#[derive(PartialEq, Clone, Copy, DebugBits, DefaultBits, FromBits)]
pub struct Config5 {
    pub pa: PhaseAdvance,
    pub smx: MaximumSpeed,
    pub spo: SpeedOutputSelection,
    pub si: SpeedIntegralGain,
}
