use core::mem::size_of;

use flash_algorithm::{ErrorCode, FlashAlgorithm, Function};
use stm32h7::stm32h753v as pac;

use crate::{
    bank::{self, Bank, ProgrammingChunk},
    FLASH_ALGO_ERROR,
};

pub struct MirroredFlasher {
    flash: pac::FLASH,
}

impl MirroredFlasher {
    fn banks(&self) -> [&pac::flash::BANK; 2] {
        [self.flash.bank1(), self.flash.bank2()]
    }

    fn init(&self) {
        for bank in self.banks() {
            let bank = Bank::new(bank);
            bank.unlock();
            bank.clear_errors();
        }
    }
}

impl FlashAlgorithm for MirroredFlasher {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        let dp = unsafe { pac::Peripherals::steal() };
        let this = Self { flash: dp.FLASH };
        this.init();
        Ok(this)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        for bank in self.banks() {
            let bank = Bank::new(bank);
            bank.erase_bank().map_err(|()| FLASH_ALGO_ERROR)?;
        }
        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        let sector_number = bank::get_sector_number(addr);
        for bank in self.banks() {
            let bank = Bank::new(bank);
            bank.erase_sector(sector_number)
                .map_err(|()| FLASH_ALGO_ERROR)?;
        }
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        for (index, bank) in self.banks().iter().enumerate() {
            let bank = Bank::new(bank);

            let bank_address_offset = index * 0x0010_0000; // 1MiB = bank size
            let base_addr = bank_address_offset + addr as usize;
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
        }
        Ok(())
    }
}

impl Drop for MirroredFlasher {
    fn drop(&mut self) {
        for bank in self.banks() {
            Bank::new(bank).lock();
        }
    }
}
