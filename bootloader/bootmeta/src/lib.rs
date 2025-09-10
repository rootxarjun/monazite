#![no_std]
#![allow(clippy::must_use_candidate)]

use core::ops::Not;

use pac::rtc::bkpr::BKPR_SPEC;
use stm32h7::Reg;
use stm32h7xx_hal::pac;
use stm32h7xx_hal::rcc::ResetReason;

#[derive(Clone, Copy, PartialEq)]
pub enum BootBank {
    Bank1 = 1,
    Bank2 = 2,
}

impl BootBank {
    /// `swap_bank` の真偽値から [`BootBank`] を構築する。
    ///
    /// 偽なら [`BootBank::Bank1`] を、真なら [`BootBank::Bank2`] を返す。
    pub fn from_swap_bank(swap_bank: bool) -> Self {
        if swap_bank {
            BootBank::Bank2
        } else {
            BootBank::Bank1
        }
    }

    /// [`BootBank`] を `swap_bank` の真偽値に変換する。
    pub fn to_swap_bank(&self) -> bool {
        match self {
            BootBank::Bank1 => false,
            BootBank::Bank2 => true,
        }
    }
}

impl Not for BootBank {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BootBank::Bank1 => BootBank::Bank2,
            BootBank::Bank2 => BootBank::Bank1,
        }
    }
}

fn serialize_next_boot_bank(next_boot_bank: Option<BootBank>) -> u32 {
    match next_boot_bank {
        None => 0,
        Some(BootBank::Bank1) => 1,
        Some(BootBank::Bank2) => 2,
    }
}

fn deserialize_next_boot_bank(bkpr_value: u32) -> Result<Option<BootBank>, u32> {
    match bkpr_value {
        0 => Ok(None),
        1 => Ok(Some(BootBank::Bank1)),
        2 => Ok(Some(BootBank::Bank2)),
        _ => Err(bkpr_value),
    }
}

struct ResetFlag {
    value: u32,
}

impl ResetFlag {
    fn new(value: u32) -> Self {
        Self { value }
    }

    fn lpwrrstf(&self) -> bool {
        self.value & (1 << 30) != 0
    }

    fn wwdg1rstf(&self) -> bool {
        self.value & (1 << 28) != 0
    }

    fn iwdg1rstf(&self) -> bool {
        self.value & (1 << 26) != 0
    }

    fn sftrstf(&self) -> bool {
        self.value & (1 << 24) != 0
    }

    fn porrstf(&self) -> bool {
        self.value & (1 << 23) != 0
    }

    fn pinrstf(&self) -> bool {
        self.value & (1 << 22) != 0
    }

    fn borrstf(&self) -> bool {
        self.value & (1 << 21) != 0
    }

    fn d2rstf(&self) -> bool {
        self.value & (1 << 20) != 0
    }

    fn d1rstf(&self) -> bool {
        self.value & (1 << 19) != 0
    }

    fn cpurstf(&self) -> bool {
        self.value & (1 << 17) != 0
    }
}

pub struct BootMeta {
    rtc: pac::RTC,
}

impl BootMeta {
    /// `BootMeta` を構築する
    pub fn new(rtc: pac::RTC) -> Self {
        Self { rtc }
    }

    fn next_boot_bank_reg(&self) -> &Reg<BKPR_SPEC> {
        &self.rtc.bkpr[0]
    }

    fn reset_flag_reg(&self) -> &Reg<BKPR_SPEC> {
        &self.rtc.bkpr[1]
    }

    /// 次回起動時のブートバンクを取得する
    ///
    /// `None` は次回起動時にも現在のブートバンクが維持されることを表す。
    ///
    /// # Errors
    /// 無効な値が読み出された場合は `Err` を返す。
    pub fn next_boot_bank(&self) -> Result<Option<BootBank>, u32> {
        let bkpr_value = self.next_boot_bank_reg().read().bits();
        deserialize_next_boot_bank(bkpr_value)
    }

    /// 次回起動時のブートバンクを設定する
    ///
    /// `None` を指定すると、次回起動時にも現在のブートバンクが維持される。
    pub fn set_next_boot_bank(&self, next_boot_bank: Option<BootBank>) {
        let bkpr_value = serialize_next_boot_bank(next_boot_bank);
        self.next_boot_bank_reg()
            .write(|w| w.bkp().bits(bkpr_value));
    }

    /// リセットフラグを読み出す
    pub fn reset_flag(&self) -> u32 {
        self.reset_flag_reg().read().bits()
    }

    /// リセットフラグを書き込む
    pub fn set_reset_flag(&self, value: u32) {
        self.reset_flag_reg().write(|w| w.bkp().bits(value));
    }

    /// リセット原因を取得する
    ///
    /// STM の HAL の enum の `ResetReason` を返す
    ///
    /// # Errors
    /// 無効な値が読み出された場合は `reset_flag` 生値を `Err` で返す。
    pub fn check_reset_source(&self, reset_flag_val: u32) -> Result<ResetReason, u32> {
        // copy from https://docs.rs/stm32h7xx-hal/latest/src/stm32h7xx_hal/rcc/reset_reason.rs.html
        let reset_flag = ResetFlag::new(reset_flag_val);
        match (
            reset_flag.lpwrrstf(),
            reset_flag.wwdg1rstf(),
            reset_flag.iwdg1rstf(),
            reset_flag.sftrstf(),
            reset_flag.porrstf(),
            reset_flag.pinrstf(),
            reset_flag.borrstf(),
            reset_flag.d2rstf(),
            reset_flag.d1rstf(),
            reset_flag.cpurstf(),
        ) {
            (false, false, false, false, true, true, true, true, true, true) => {
                Ok(ResetReason::PowerOnReset)
            }
            (false, false, false, false, false, true, false, false, false, true) => {
                Ok(ResetReason::PinReset)
            }
            (false, false, false, false, false, true, true, false, false, true) => {
                Ok(ResetReason::BrownoutReset)
            }
            (false, false, false, true, false, true, false, false, false, true) => {
                Ok(ResetReason::SystemReset)
            }
            (false, false, false, false, false, false, false, false, false, true) => {
                Ok(ResetReason::CpuReset)
            }
            (false, true, false, false, false, false, false, false, false, false)
            | (false, true, false, false, false, true, false, false, false, true) => {
                // コピペ元の HAL を見る限り、リファレンスの表で太字になっている1が両方0でもこのケースと判定していいらしい（リファレンスマニュアルに明記はされていない）
                Ok(ResetReason::WindowWatchdogReset)
            }
            (false, false, true, false, false, true, false, false, false, true) => {
                Ok(ResetReason::IndependentWatchdogReset)
            }
            (false, true, true, false, false, true, false, false, false, true) => {
                // おそらくフラグをリセットせずに WWDG1 と IWDG1 が連続して発火したケースに対応（リファレンスマニュアルに明記はされていない）
                Ok(ResetReason::GenericWatchdogReset)
            }
            (false, false, false, false, false, false, false, false, true, false) => {
                Ok(ResetReason::D1ExitsDStandbyMode)
            }
            (false, false, false, false, false, false, false, true, false, false) => {
                Ok(ResetReason::D2ExitsDStandbyMode)
            }
            (true, false, false, false, false, true, false, false, false, true) => {
                Ok(ResetReason::D1EntersDStandbyErroneouslyOrCpuEntersCStopErroneously)
            }
            _ => Err(reset_flag_val),
        }
    }
}

pub struct FlashOptionBytes(core::marker::PhantomData<()>);

impl FlashOptionBytes {
    /// # Safety
    /// TODO
    pub unsafe fn new() -> FlashOptionBytes {
        FlashOptionBytes(core::marker::PhantomData)
    }

    // Using &self limits the scope of calls to unsafe.
    #[allow(clippy::unused_self)]
    fn flash(&self) -> &pac::flash::RegisterBlock {
        unsafe { &*pac::FLASH::ptr() }
    }

    /// 現在の起動時の `SWAP_BANK` の値を返す
    ///
    /// `FLASH_OPTCR` の `SWAP_BANK` ビットを読み出すため、[`Self::write_swap_bank`] と [`Self::program`] を実行しても、
    /// リセットまではこのメソッドの返り値は変化しない。
    pub fn read_swap_bank(&self) -> bool {
        self.flash().optcr().read().swap_bank().bit_is_set()
    }

    /// Flash のオプションバイトのロックを解除する
    pub fn unlock(&self) {
        const FLASH_OPT_KEY1: u32 = 0x0819_2A3B;
        const FLASH_OPT_KEY2: u32 = 0x4C5D_6E7F;

        cortex_m::interrupt::free(|_cs| {
            self.flash()
                .optkeyr()
                .write(|w| w.optkeyr().variant(FLASH_OPT_KEY1));
            self.flash()
                .optkeyr()
                .write(|w| w.optkeyr().variant(FLASH_OPT_KEY2));
        });
    }

    /// Flash のオプションバイトをロックする
    pub fn lock(&self) {
        cortex_m::interrupt::free(|_cs| {
            self.flash().optcr().modify(|_, w| w.optlock().set_bit());
        });
    }

    /// Flash のオプションバイトを永続化する
    pub fn program(&self) {
        cortex_m::interrupt::free(|_cs| {
            self.flash().optcr().modify(|_, w| w.optstart().set_bit());
            while self.flash().optsr_cur().read().opt_busy().bit_is_set() {
                cortex_m::asm::nop();
            }
        });
    }

    /// `SWAP_BANK` の値を書き込む
    ///
    /// [`Self::program`] を実行するまで、値は永続化されない。
    pub fn write_swap_bank(&self, swap_bank: bool) {
        cortex_m::interrupt::free(|_cs| {
            self.unlock();
            self.flash()
                .optsr_prg()
                .modify(|_, w| w.swap_bank_opt().bit(swap_bank));
            self.program();
            self.lock();
        });
    }
}
