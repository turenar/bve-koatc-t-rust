mod bve;

use crate::bve::{
    Beacon, ConstantSpeed, HandleInitialPosition, Handles, Horn, Key, NotchPosition,
    ReverserPosition, VehicleSpec, VehicleState,
};
use std::ffi::c_int;

#[no_mangle]
pub extern "system" fn Load() {}
#[no_mangle]
pub extern "system" fn Dispose() {}

#[no_mangle]
pub extern "system" fn SetVehicleSpec(_spec: VehicleSpec) {}
// Called when the game is started
#[no_mangle]
pub extern "system" fn Initialize(_brake: HandleInitialPosition) {}
#[no_mangle]
pub extern "system" fn Elapse(
    _state: VehicleState,
    _panel: &mut [c_int; 256],
    _sound: &mut [c_int; 256],
) -> Handles {
    Handles {
        power: NotchPosition::NEUTRAL,
        brake: NotchPosition::NEUTRAL,
        reverser: ReverserPosition(0),
        constant_speed: ConstantSpeed::Disable,
    }
}
#[no_mangle]
pub extern "system" fn SetPower(_power: c_int) {}
#[no_mangle]
pub extern "system" fn SetBrake(_brake: NotchPosition) {}
#[no_mangle]
pub extern "system" fn SetReverser(_reverser: ReverserPosition) {}
#[no_mangle]
pub extern "system" fn KeyDown(_key: Key) {}
#[no_mangle]
pub extern "system" fn KeyUp(key: Key) {}
#[no_mangle]
pub extern "system" fn HornBlow(_horn: Horn) {}
#[no_mangle]
pub extern "system" fn DoorOpen() {}
#[no_mangle]
pub extern "system" fn DoorClose() {}
#[no_mangle]
pub extern "system" fn SetSignal(_signal: c_int) {}
#[no_mangle]
pub extern "system" fn SetBeaconData(_beacon: Beacon) {}
