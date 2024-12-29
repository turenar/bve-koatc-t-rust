use crate::bve::unit::{Length, Time};
use std::ffi::c_double;
use std::ffi::c_float;
use std::ffi::c_int;

#[repr(C)]
#[derive(Debug)]
pub struct BeaconType(pub u32);
#[derive(Debug)]
pub struct PanelId(pub u8);
#[derive(Debug)]
pub struct SoundId(pub u8);
#[repr(C)]
pub enum Key {
    S = 0,
    A1 = 1,
    A2 = 2,
    B1 = 3,
    B2 = 4,
    C1 = 5,
    C2 = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
}
#[repr(C)]
pub enum HandleInitialPosition {
    HandleRemoved = 2,
    EmergencyBrake = 1,
    ServiceBrake = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SoundControl {
    Stop = -10000,
    Play = 1,
    PlayLooping = 0,
    Continue = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum Horn {
    Primary = 0,
    Secondary = 1,
    Music = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ConstantSpeed {
    Continue = 0,
    Enable = 1,
    Disable = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct NotchPosition(pub i32);
impl NotchPosition {
    pub const NEUTRAL: NotchPosition = NotchPosition(0);
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NotchCount(pub i32);
impl NotchCount {
    pub fn full(&self) -> NotchPosition {
        NotchPosition(self.0)
    }
    pub fn half(&self) -> NotchPosition {
        NotchPosition(self.0 / 2)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct VehicleSpec {
    brake_notches: NotchCount, // Number of Brake Notches
    power_notches: NotchCount, // Number of Power Notches
    ats_notch: NotchPosition,  // ATS Cancel Notch
    b67_notch: NotchPosition,  // 80% Brake (67 degree)
    cars: c_int,               // Number of Cars
}
#[repr(C)]
#[derive(Debug)]
pub struct VehicleState {
    location: c_double,
    speed: c_float,
    time: Time<c_int>,
    bc_pressure: Pressure,
    mr_pressure: Pressure,
    er_pressure: Pressure,
    bp_pressure: Pressure,
    sap_pressure: Pressure,
    current: c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct Pressure(pub c_int);

// Received Data from Beacon
#[repr(C)]
#[derive(Debug)]
pub struct Beacon {
    pub beacon_type: BeaconType,
    pub signal: c_int,
    pub distance: Length<c_float>,
    pub optional: c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct ReverserPosition(pub c_int);
#[repr(C)]
#[derive(Debug)]
pub struct Handles {
    pub brake: NotchPosition,
    pub power: NotchPosition,
    pub reverser: ReverserPosition,
    pub constant_speed: ConstantSpeed,
}
