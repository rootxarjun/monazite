#![no_main]
#![no_std]

use core::ops::Range;

use bootmeta::{BootBank, FlashOptionBytes};
use core::mem::size_of;
use defmt_rtt as _;
use fugit::MillisDurationU32;
use panic_probe as _;
use stm32h7xx_hal::{independent_watchdog::IndependentWatchdog, pac};

// ウォッチドッグのタイムアウト時間
const WDT_TIMEOUT: MillisDurationU32 = MillisDurationU32::secs(20);

// リセット直後の CPU は HSI の 64MHz で駆動されている
const CYCLES_PER_SECOND: u32 = 64_000_000;

const AXI_SRAM: Range<usize> = 0x2400_0000..0x2408_0000;
const SRAM1: Range<usize> = 0x3000_0000..0x3002_0000;
const SRAM2: Range<usize> = 0x3002_0000..0x3004_0000;
const SRAM3: Range<usize> = 0x3004_0000..0x3004_8000;
const SRAM4: Range<usize> = 0x3800_0000..0x3801_0000;

#[cortex_m_rt::entry]
#[allow(clippy::mut_mut)]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 真っ先にウォッチドッグを有効化する
    let mut iwdg = IndependentWatchdog::new(dp.IWDG);
    iwdg.start(WDT_TIMEOUT);

    // Backup Domain を有効化し、Backup Register をアクセス可能にする
    dp.PWR.cr1.modify(|_, w| w.dbp().set_bit());
    while dp.PWR.cr1.read().dbp().bit_is_clear() {}

    // Backup Domain の有効化に時間がかかっている可能性があるので、ここでウォッチドッグをフィードする
    iwdg.feed();

    let bootmeta = bootmeta::BootMeta::new(dp.RTC);
    let flop = unsafe { FlashOptionBytes::new() };

    // Flash のオプションバイトを読み出し、現在のブートバンクを取得する
    let flop_swap_bank = flop.read_swap_bank();
    let current_boot_bank = BootBank::from_swap_bank(flop_swap_bank);

    // リセットフラグを読み出し、Backup Register に保存する
    bootmeta.set_reset_flag(dp.RCC.rsr.read().bits());

    // ブートすべきバンクを取得する
    let desired_boot_bank = if dp.RCC.rsr.read().borrstf().is_reset_occourred() {
        // パワーオンリセット・ブラウンアウトリセットが発生した場合 Backup Register は不定なため、ブートバンクの変更がないものとして扱う
        None
    } else {
        // WDTによるリセット・システムリセットの場合は Backup Register からブートバンクを取得する
        // 不正な値が読み出された場合は None に潰し、ブートバンクの変更がないものとして扱う
        bootmeta.next_boot_bank().unwrap_or(None)
    };
    // リセットフラグをクリア
    dp.RCC.rsr.modify(|_, w| w.rmvf().set_bit());

    if let Some(desired_boot_bank) = desired_boot_bank {
        if desired_boot_bank != current_boot_bank {
            // ブートすべきバンクが指定されており、それが現在のブートバンクと異なる場合

            // 待機の前にウォッチドッグをフィードする
            iwdg.feed();
            // リブートループで内蔵 Flash が高速に消耗するのを防ぐため、1秒待機する
            cortex_m::asm::delay(CYCLES_PER_SECOND);

            // ブートバンクを切り替える
            flop.write_swap_bank(desired_boot_bank.to_swap_bank());
            cortex_m::peripheral::SCB::sys_reset();
            // ここには到達しない
        }
    }

    // ブートに失敗した場合に裏のバンクに切り替えるため、次回ブートバンクを現在の裏のバンクに設定する
    // ブートが成功した場合はアプリケーションの責任で next_boot_bank を None に戻すことになっている
    let rolled_back_boot_bank = !current_boot_bank;
    bootmeta.set_next_boot_bank(rolled_back_boot_bank.into());

    // 内蔵 RAM のゼロクリアに時間がかかる可能性があるので、事前にウォッチドッグをフィードする
    iwdg.feed();

    // SRAM1, SRAM2, SRAM3 を有効化する
    enable_sram123(&dp.RCC);

    // 内蔵RAMをゼロクリアする
    fill_u64(AXI_SRAM, 0u64);
    fill_u32(SRAM1, 0u32);
    fill_u32(SRAM2, 0u32);
    fill_u32(SRAM3, 0u32);
    fill_u32(SRAM4, 0u32);

    // コードジャンプ直前にウォッチドッグをフィードする
    iwdg.feed();
    unsafe {
        // Arm の Vector Table をアプリケーションのものに書き換える
        cp.SCB.vtor.write(0x0800_0000);
        // アプリケーションのコードにジャンプする
        cortex_m::asm::bootload(0x0800_0000 as *const u32);
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

fn enable_sram123(rcc: &pac::RCC) {
    rcc.ahb2enr.modify(|_, w| {
        w.sram1en()
            .set_bit()
            .sram2en()
            .set_bit()
            .sram3en()
            .set_bit()
    });
}

fn fill_u64(range: Range<usize>, pat: u64) {
    for addr in range.step_by(size_of::<u64>()) {
        let ptr = addr as *mut u64;
        // fill を使わず volatile で個別に書き込むことで、最適化によるコード除去を防ぐ
        unsafe { ptr.write_volatile(pat) };
    }
}

fn fill_u32(range: Range<usize>, pat: u32) {
    for addr in range.step_by(size_of::<u32>()) {
        let ptr = addr as *mut u32;
        // fill を使わず volatile で個別に書き込むことで、最適化によるコード除去を防ぐ
        unsafe { ptr.write_volatile(pat) };
    }
}
