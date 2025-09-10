use std::{process, sync::Mutex};

use c2a_monazite_btmgr_bind::{BootBank, Btmgr as BtmgrBind};

pub struct Btmgr {
    next_boot_bank: Mutex<Option<BootBank>>,
}

impl Btmgr {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_boot_bank: Mutex::new(None),
        }
    }
}

impl BtmgrBind for Btmgr {
    fn get_current_boot_bank(&self) -> BootBank {
        BootBank::Bank1
    }

    fn get_next_boot_bank(&self) -> Option<BootBank> {
        *self.next_boot_bank.lock().unwrap()
    }

    fn set_next_boot_bank(&self, next_boot_bank: Option<BootBank>) {
        *self.next_boot_bank.lock().unwrap() = next_boot_bank;
    }

    fn get_reset_flag(&self) -> u32 {
        0
    }

    fn get_reset_reason(&self) -> i32 {
        -1
    }

    fn system_reset(&self) -> ! {
        process::exit(0)
    }
}
