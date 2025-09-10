use core::mem::size_of;

use flash_algorithm::{ErrorCode, FlashAlgorithm, Function};
use stm32h7::stm32h753v as pac;

use crate::{
    bank::{self, Bank, ProgrammingChunk},
    FLASH_ALGO_ERROR,
};

const FLASH_OPTKEY1: u32 = 0x0819_2A3B; // Flash option byte key1
const FLASH_OPTKEY2: u32 = 0x4C5D_6E7F; // Flash option byte key2: used with FLASH_OPTKEY1

pub struct OneSideFlasher<S> {
    flash: pac::FLASH,
    rtc: pac::RTC,
    _bank_select: core::marker::PhantomData<S>,
}

impl<S> OneSideFlasher<S> {
    fn effective_swap_bank(&self) -> bool {
        self.flash.optcr().read().swap_bank().bit_is_set()
    }

    fn absolute_banks(&self) -> (&pac::flash::BANK, &pac::flash::BANK) {
        if self.effective_swap_bank() {
            (self.flash.bank2(), self.flash.bank1())
        } else {
            (self.flash.bank1(), self.flash.bank2())
        }
    }

    fn write_swap_bank(&self, swap_bank: bool) {
        // Unlock
        self.flash
            .optkeyr()
            .write(|w| w.optkeyr().variant(FLASH_OPTKEY1));
        self.flash
            .optkeyr()
            .write(|w| w.optkeyr().variant(FLASH_OPTKEY2));

        // Write swap bank
        self.flash
            .optsr_prg()
            .modify(|_, w| w.swap_bank_opt().bit(swap_bank));
        // Start programming
        self.flash.optcr().modify(|_, w| w.optstart().set_bit());
        // Wait for completion
        while self.flash.optsr_cur().read().opt_busy().bit_is_set() {}

        // Lock
        self.flash.optcr().modify(|_, w| w.optlock().set_bit());
    }
}

impl<S> OneSideFlasher<S>
where
    S: BankSelect + 'static,
{
    fn bank(&self) -> &pac::flash::BANK {
        let (bank1, bank2) = self.absolute_banks();
        if S::SWAP_BANK {
            bank2
        } else {
            bank1
        }
    }

    fn bank_address_offset(&self) -> usize {
        if S::SWAP_BANK == self.effective_swap_bank() {
            0
        } else {
            0x0010_0000 // 1MiB = bank size
        }
    }

    fn clear_next_boot_bank(&self) {
        // ブートローダーの仕様と結託した挙動
        self.rtc.bkpr[0].write(|w| w.bkp().bits(0));
    }

    fn init(&self) {
        let bank = Bank::new(self.bank());
        bank.unlock();
        bank.clear_errors();
        if S::SWAP_BANK != self.effective_swap_bank() {
            self.write_swap_bank(S::SWAP_BANK);
        }
        self.clear_next_boot_bank();
    }
}

impl<S> FlashAlgorithm for OneSideFlasher<S>
where
    S: BankSelect + 'static,
{
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        let dp = unsafe { pac::Peripherals::steal() };
        let this = Self {
            flash: dp.FLASH,
            rtc: dp.RTC,
            _bank_select: core::marker::PhantomData,
        };
        this.init();
        Ok(this)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        let bank = Bank::new(self.bank());
        bank.erase_bank().map_err(|()| FLASH_ALGO_ERROR)
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        let sector_number = bank::get_sector_number(addr);
        let bank = Bank::new(self.bank());
        bank.erase_sector(sector_number)
            .map_err(|()| FLASH_ALGO_ERROR)
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        let bank = Bank::new(self.bank());

        let base_addr = self.bank_address_offset() + addr as usize;
        if data.as_ptr().align_offset(size_of::<u32>()) != 0 {
            // data must be 32-bit aligned
            return Err(FLASH_ALGO_ERROR);
        }
        if data.len() % 32 != 0 {
            // data must be 32-byte(256-bit) aligned
            return Err(FLASH_ALGO_ERROR);
        }
        let num_chunks = data.len() / 32;
        let chunks = unsafe {
            // Safety: data is 32-byte aligned
            #[allow(clippy::cast_ptr_alignment)]
            core::slice::from_raw_parts(data.as_ptr().cast::<ProgrammingChunk>(), num_chunks)
        };
        for (index, chunk) in chunks.iter().enumerate() {
            bank.program_chunk(base_addr + (index * size_of::<ProgrammingChunk>()), chunk)
                .map_err(|()| FLASH_ALGO_ERROR)?;
        }
        Ok(())
    }
}

impl<T> Drop for OneSideFlasher<T> {
    fn drop(&mut self) {
        Bank::new(self.flash.bank1()).lock();
        Bank::new(self.flash.bank2()).lock();
    }
}

pub trait BankSelect {
    const SWAP_BANK: bool;
}

pub enum Bank1 {}
impl BankSelect for Bank1 {
    const SWAP_BANK: bool = false;
}
pub enum Bank2 {}
impl BankSelect for Bank2 {
    const SWAP_BANK: bool = true;
}
