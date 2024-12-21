use crate::bve::structure::{PanelId, SoundControl, SoundId};
use crate::bve::{
    Beacon, Handles, Horn, Key, NotchPosition, ReverserPosition, VehicleSpec, VehicleState,
};
use std::ffi::c_int;

pub struct PanelSound<'a> {
    panel: &'a mut [c_int; 256],
    sound: &'a mut [SoundControl; 256],
}

impl<'a> PanelSound<'a> {
    pub fn new(panel: &'a mut [c_int; 256], sound: &'a mut [SoundControl; 256]) -> Self {
        Self { panel, sound }
    }
    pub fn set_panel(&mut self, panel: PanelId, value: c_int) {
        self.panel[panel.0 as usize] = value;
    }
    pub fn set_sound(&mut self, sound: SoundId, value: SoundControl) {
        self.sound[sound.0 as usize] = value;
    }
}

pub trait AtsModule {
    fn set_vehicle_spec(spec: &VehicleSpec);
    fn tick(state: &VehicleState) -> Handles;
    fn power(power: NotchPosition);
    fn brake(brake: NotchPosition);
    fn reverser(reverser: ReverserPosition);
    fn key_down(_key: Key) {}
    fn key_up(_key: Key) {}
    fn horn_brow(_horn: Horn) {}
    fn open_door() {}
    fn close_door() {}
    fn set_signal(_signal: c_int) {}
    fn receive_beacon(beacon: &Beacon);
}
