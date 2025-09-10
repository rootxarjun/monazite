use core::mem::size_of;

use cortex_m::asm::{dsb, isb, nop};
use stm32h7::stm32h753v as pac;

const FLASH_BANK_SIZE: u32 = 0x20000; // 128 KiB
const FLASH_KEY1: u32 = 0x4567_0123; // Flash key1
const FLASH_KEY2: u32 = 0xCDEF_89AB; // Flash key2: used with FLASH_KEY1 to unlock the FLASH registers access
const FLASH_PGERR: u32 = 0x0FEF;

pub fn get_sector_number(addr: u32) -> u8 {
    ((addr / FLASH_BANK_SIZE) & 0xFF) as u8
}

pub type ProgrammingChunk = [u32; 8];

pub struct Bank<'a> {
    reg: &'a pac::flash::BANK,
}

impl<'a> Bank<'a> {
    pub fn new(reg: &'a pac::flash::BANK) -> Self {
        Self { reg }
    }

    pub fn unlock(&self) {
        self.reg.keyr.write(|w| w.keyr().variant(FLASH_KEY1));
        self.reg.keyr.write(|w| w.keyr().variant(FLASH_KEY2));
    }

    pub fn lock(&self) {
        self.reg.cr.modify(|_, w| w.lock().set_bit());
    }

    pub fn clear_errors(&self) {
        self.reg.sr.write(|w| unsafe { w.bits(FLASH_PGERR) });
    }

    pub fn erase_bank(&self) -> Result<(), ()> {
        self.clear_errors();
        self.reg.cr.write(|w| {
            w.psize().variant(3); // 64-bit
            w.ber().set_bit(); // Bank erase request
            w
        });
        self.start_operation();
        self.wait_for_completion()
    }

    pub fn erase_sector(&self, sector_number: u8) -> Result<(), ()> {
        self.clear_errors();
        self.reg.cr.write(|w| {
            w.psize().variant(3); // 64-bit
            w.ser().set_bit(); // Sector erase request
            w.snb().variant(sector_number);
            w
        });
        self.start_operation();
        self.wait_for_completion()
    }

    pub fn program_chunk(&self, addr: usize, chunk: &ProgrammingChunk) -> Result<(), ()> {
        self.clear_errors();
        self.reg.cr.write(|w| {
            w.psize().variant(3); // 64-bit
            w.pg().set_bit(); // Program enable
            w
        });
        isb();
        dsb();

        for (index, word) in chunk.iter().enumerate() {
            let dest_addr = addr + (index * size_of::<u32>());
            unsafe {
                core::ptr::write_volatile(dest_addr as *mut u32, *word);
            }
        }
        isb();
        dsb();

        self.wait_for_completion()
    }

    fn start_operation(&self) {
        self.reg.cr.modify(|_, w| w.start().set_bit());
        isb();
        dsb();
    }

    fn wait_for_completion(&self) -> Result<(), ()> {
        while self.reg.sr.read().bsy().bit_is_set() {
            nop();
        }
        if self.reg.sr.read().pgserr().bit_is_set() {
            return Err(());
        }
        Ok(())
    }
}
