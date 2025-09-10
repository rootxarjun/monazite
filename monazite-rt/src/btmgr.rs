use bootmeta::{BootBank as MetaBootBank, BootMeta, FlashOptionBytes};
use c2a_monazite_btmgr_bind::{BootBank, Btmgr as BtmgrBind};
use cortex_m::interrupt::Mutex;
use stm32h7xx_hal::rcc::ResetReason;

pub struct Btmgr {
    bootmeta: Mutex<BootMeta>,
    flash_option_bytes: FlashOptionBytes,
}

fn to_meta_boot_bank(boot_bank: BootBank) -> MetaBootBank {
    match boot_bank {
        BootBank::Bank1 => MetaBootBank::Bank1,
        BootBank::Bank2 => MetaBootBank::Bank2,
    }
}

fn from_meta_boot_bank(meta_boot_bank: MetaBootBank) -> BootBank {
    match meta_boot_bank {
        MetaBootBank::Bank1 => BootBank::Bank1,
        MetaBootBank::Bank2 => BootBank::Bank2,
    }
}

fn from_reset_reason(reset_reason: ResetReason) -> i32 {
    match reset_reason {
        ResetReason::PowerOnReset => 0,
        ResetReason::PinReset => 1,
        ResetReason::BrownoutReset => 2,
        ResetReason::SystemReset => 3,
        ResetReason::CpuReset => 4,
        ResetReason::WindowWatchdogReset => 5,
        ResetReason::IndependentWatchdogReset => 6,
        ResetReason::GenericWatchdogReset => 7,
        ResetReason::D1ExitsDStandbyMode => 8,
        ResetReason::D2ExitsDStandbyMode => 9,
        ResetReason::D1EntersDStandbyErroneouslyOrCpuEntersCStopErroneously => 10,
        ResetReason::Unknown { .. } => -1,
    }
}

impl Btmgr {
    pub fn new(bootmeta: BootMeta, flash_option_bytes: FlashOptionBytes) -> Self {
        Self {
            bootmeta: Mutex::new(bootmeta),
            flash_option_bytes,
        }
    }
}

impl BtmgrBind for Btmgr {
    fn get_current_boot_bank(&self) -> BootBank {
        from_meta_boot_bank(MetaBootBank::from_swap_bank(
            self.flash_option_bytes.read_swap_bank(),
        ))
    }

    fn get_next_boot_bank(&self) -> Option<BootBank> {
        cortex_m::interrupt::free(|cs| {
            let bootmeta = self.bootmeta.borrow(cs);
            // 無効な値は None に潰す
            // これはブートローダーと一貫性のある挙動
            bootmeta
                .next_boot_bank()
                .unwrap_or(None)
                .map(from_meta_boot_bank)
        })
    }

    fn set_next_boot_bank(&self, next_boot_bank: Option<BootBank>) {
        cortex_m::interrupt::free(|cs| {
            let bootmeta = self.bootmeta.borrow(cs);
            bootmeta.set_next_boot_bank(next_boot_bank.map(to_meta_boot_bank));
        });
    }

    fn get_reset_flag(&self) -> u32 {
        cortex_m::interrupt::free(|cs| {
            let bootmeta = self.bootmeta.borrow(cs);
            bootmeta.reset_flag()
        })
    }

    fn get_reset_reason(&self) -> i32 {
        cortex_m::interrupt::free(|cs| {
            let bootmeta = self.bootmeta.borrow(cs);
            match bootmeta.check_reset_source(bootmeta.reset_flag()) {
                Ok(reason) => from_reset_reason(reason),
                Err(_) => -1,
            }
        })
    }

    fn system_reset(&self) -> ! {
        cortex_m::peripheral::SCB::sys_reset();
    }
}
