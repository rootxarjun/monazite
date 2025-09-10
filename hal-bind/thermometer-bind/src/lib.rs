#![no_std]

mod bind;

use core::ffi::c_float;

use atomic_once_cell::AtomicOnceCell;

pub trait Thermometer: Sync {
    fn value(&self) -> f32;
}

#[no_mangle]
pub static C2A_MONAZITE_THERMOMETER: AtomicOnceCell<&'static dyn Thermometer> =
    AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn THERMOMETER_get_value() -> c_float {
    let thermometer = C2A_MONAZITE_THERMOMETER.get();
    thermometer.value()
}
