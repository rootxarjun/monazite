#![no_std]

use c2a_core::hal::wdt as bind;

use core::ffi::c_int;

use atomic_once_cell::AtomicOnceCell;

pub trait Wdt: Sync {
    fn initialize(&self);
    fn clear(&self);
    fn enable(&self, time: u32);
}

#[no_mangle]
pub static C2A_MONAZITE_WDT: AtomicOnceCell<&'static dyn Wdt> = AtomicOnceCell::new();

/// # Safety
/// `wdt_config` は [`bind::WDT_Config`] を指す有効なポインタでなければならない
#[no_mangle]
pub unsafe extern "C" fn WDT_initialize(wdt_config: *mut bind::WDT_Config) -> c_int {
    // monazite では bootloader の先頭時点で WDT を20秒で有効化する
    // bootloader 無しでの開発中も WDT が有効でないとテレメに整合性がないため、ここで再度有効化する
    WDT_set_timer(wdt_config, 20_000);
    WDT_enable(wdt_config);
    0
}

/// # Safety
/// `wdt_config` は [`bind::WDT_Config`] を指す有効なポインタでなければならない
#[no_mangle]
pub unsafe extern "C" fn WDT_clear(wdt_config: *mut bind::WDT_Config) -> c_int {
    let wdt = C2A_MONAZITE_WDT.get();
    let config = &mut *wdt_config;
    if config.is_clear_enable == 1 {
        wdt.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn WDT_disable(_wdt_config: *mut bind::WDT_Config) -> c_int {
    let _wdt = C2A_MONAZITE_WDT.get();
    debug_assert!(false, "WDT can't be disabled on monazite");
    0
}

/// # Safety
/// `wdt_config` は [`bind::WDT_Config`] を指す有効なポインタでなければならない
#[no_mangle]
pub unsafe extern "C" fn WDT_enable(wdt_config: *mut bind::WDT_Config) -> c_int {
    let wdt = C2A_MONAZITE_WDT.get();
    let config = &mut *wdt_config;
    #[allow(clippy::cast_sign_loss)]
    wdt.enable(config.timer_setting as u32);
    config.is_clear_enable = 1;
    config.is_wdt_enable = 1;
    0
}

/// # Safety
/// `wdt_config` は [`bind::WDT_Config`] を指す有効なポインタでなければならない
#[no_mangle]
pub unsafe extern "C" fn WDT_set_timer(wdt_config: *mut bind::WDT_Config, time: c_int) -> c_int {
    let wdt = C2A_MONAZITE_WDT.get();
    let config = &mut *wdt_config;
    config.timer_setting = time;
    if config.is_wdt_enable == 1 {
        #[allow(clippy::cast_sign_loss)]
        wdt.enable(config.timer_setting as u32);
    }
    0
}
