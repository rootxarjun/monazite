#![no_std]

mod bind;

use core::ffi::{c_int, c_uint};

use atomic_once_cell::AtomicOnceCell;

#[allow(clippy::cast_possible_wrap)]
#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum BootBank {
    Bank1 = bind::BTMGR_BOOT_BANK_BTMGR_BANK_1.0 as i32,
    Bank2 = bind::BTMGR_BOOT_BANK_BTMGR_BANK_2.0 as i32,
}

pub trait Btmgr: Sync {
    fn get_current_boot_bank(&self) -> BootBank;
    fn get_next_boot_bank(&self) -> Option<BootBank>;
    fn set_next_boot_bank(&self, next_boot_bank: Option<BootBank>);
    fn get_reset_flag(&self) -> u32;
    fn get_reset_reason(&self) -> i32;
    fn system_reset(&self) -> !;
}

#[no_mangle]
pub static C2A_MONAZITE_BTMGR: AtomicOnceCell<&'static dyn Btmgr> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn BTMGR_get_current_boot_bank() -> c_int {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    btmgr.get_current_boot_bank() as i32
}

#[no_mangle]
pub extern "C" fn BTMGR_get_next_boot_bank() -> c_int {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    match btmgr.get_next_boot_bank() {
        Some(bank) => bank as i32,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn BTMGR_set_next_boot_bank(next_boot_bank: c_int) -> c_int {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    let next_boot_bank = match next_boot_bank {
        0 => None,
        1 => Some(BootBank::Bank1),
        2 => Some(BootBank::Bank2),
        _ => return bind::BTMGR_BOOT_ERR_CODE_BTMGR_INVALID_PARAM_ERR.0,
    };
    btmgr.set_next_boot_bank(next_boot_bank);
    bind::BTMGR_BOOT_ERR_CODE_BTMGR_OK.0
}

#[no_mangle]
pub extern "C" fn BTMGR_get_reset_flag() -> c_uint {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    btmgr.get_reset_flag() as c_uint
}

#[no_mangle]
pub extern "C" fn BTMGR_get_reset_reason() -> c_int {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    btmgr.get_reset_reason()
}

#[no_mangle]
pub extern "C" fn BTMGR_system_reset() {
    let btmgr = C2A_MONAZITE_BTMGR.get();
    btmgr.system_reset();
}
