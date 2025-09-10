#![no_std]

mod bind;

use core::ffi::c_uint;

use atomic_once_cell::AtomicOnceCell;

pub trait Ramecc: Sync {
    fn scrubbing_loops(&self) -> u32;
    fn single_errors(&self) -> u32;
    fn double_errors(&self) -> u32;
    fn double_errors_on_byte_write(&self) -> u32;
    fn dtcm_single_errors(&self) -> u32;
    fn dtcm_double_errors(&self) -> u32;
    fn dtcm_double_errors_on_byte_write(&self) -> u32;
    fn scrubbing_interval(&self) -> u32;
    fn set_scrubbing_interval(&self, scrubbing_interval_tick: u32);
}

#[no_mangle]
pub static C2A_MONAZITE_RAMECC: AtomicOnceCell<&'static dyn Ramecc> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn RAMECC_get_scrubbing_loop() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.scrubbing_loops()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_single_error() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.single_errors()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_double_error() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.double_errors()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_double_error_on_byte_write() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.double_errors_on_byte_write()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_dtcm_single_error() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.dtcm_single_errors()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_dtcm_double_error() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.dtcm_double_errors()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_dtcm_double_error_on_byte_write() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.dtcm_double_errors_on_byte_write()
}

#[no_mangle]
pub extern "C" fn RAMECC_get_scrubbing_interval() -> c_uint {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.scrubbing_interval()
}

#[no_mangle]
pub extern "C" fn RAMECC_set_scrubbing_interval(scrubbing_interval_tick: c_uint) {
    let ramecc = C2A_MONAZITE_RAMECC.get();
    ramecc.set_scrubbing_interval(scrubbing_interval_tick);
}
