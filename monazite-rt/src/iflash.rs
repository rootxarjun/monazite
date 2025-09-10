use core::{cell::RefCell, convert::Infallible};

use c2a_monazite_iflash_bind::{Error, Iflash as IflashBind};
use cortex_m::interrupt::Mutex;
use stm32h7xx_hal::pac::FLASH;

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Idle { last_error: Option<Error> },
    Erasing(u8),
    Programming { dest: usize, src: usize, len: usize },
}

struct Inner {
    buffer: &'static mut [u8],
    flash: FLASH,
    state: State,
}

impl Inner {
    const UNLOCK_KEY1: u32 = 0x4567_0123;
    const UNLOCK_KEY2: u32 = 0xCDEF_89AB;

    const SECTOR_FIRST: u8 = 0;
    const SECTOR_LAST: u8 = 6; // sector 7 if for the bootloader

    const FLASH_BANK2_BASE: usize = 0x0810_0000;
    const FLASH_BANK_SIZE: usize = 1024 * 1024; // 1MB
    const FLASH_ROW_SIZE: usize = 32; // 256bit

    fn new(flash: FLASH, buffer: &'static mut [u8]) -> Self {
        Inner {
            buffer,
            flash,
            state: State::Idle { last_error: None },
        }
    }

    fn unlock(&self) {
        let bank = self.flash.bank2();
        bank.keyr.write(|w| w.keyr().variant(Self::UNLOCK_KEY1));
        bank.keyr.write(|w| w.keyr().variant(Self::UNLOCK_KEY2));
        assert!(!bank.cr.read().lock().bit());
    }

    pub fn poll(&mut self) {
        match self.state {
            State::Idle { .. } => {}
            State::Erasing(sector) => {
                let eop = self.flash.bank2().sr.read().eop().bit_is_set();
                self.flash.bank2().ccr.write(|w| w.clr_eop().set_bit());
                self.flash.bank2().cr.modify(|_, w| {
                    w.eopie().clear_bit();
                    w.ser().clear_bit();
                    w.snb().variant(0);
                    w.pg().clear_bit();
                    w.lock().set_bit();
                    w
                });
                if eop {
                    if sector == Self::SECTOR_LAST {
                        self.state = State::Idle { last_error: None };
                    } else {
                        self.start_erase_sector(sector + 1);
                    }
                }
            }
            State::Programming { dest, src, len } => {
                let eop = self.flash.bank2().sr.read().eop().bit_is_set();
                self.flash.bank2().ccr.write(|w| w.clr_eop().set_bit());
                self.flash.bank2().cr.modify(|_, w| {
                    w.eopie().clear_bit();
                    w.psize().variant(0b00);
                    w.ser().clear_bit();
                    w.pg().clear_bit();
                    w.lock().set_bit();
                    w
                });
                if eop {
                    let dest = dest + Self::FLASH_ROW_SIZE;
                    let src = src + Self::FLASH_ROW_SIZE;
                    let len = len - Self::FLASH_ROW_SIZE;
                    if len == 0 {
                        self.state = State::Idle { last_error: None };
                    } else {
                        self.start_program_row(dest, src, len);
                    }
                }
            }
        }
    }

    fn start_erase_sector(&mut self, sector: u8) {
        self.unlock();
        self.flash.bank2().cr.modify(|_, w| {
            w.eopie().set_bit(); // enable end-of-operation interrupt
            w.start().set_bit(); // start operation
            w.ser().set_bit(); // sector erase
            w.snb().variant(sector); // sector 0
            w.pg().clear_bit(); // not programming
            w
        });
        self.state = State::Erasing(sector);
    }

    pub fn erase(&mut self) -> nb::Result<(), Infallible> {
        if let State::Idle { .. } = self.state {
            self.start_erase_sector(Self::SECTOR_FIRST);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn start_program_row(&mut self, dest: usize, src: usize, len: usize) {
        self.unlock();
        self.flash.bank2().cr.modify(|_, w| {
            w.eopie().set_bit(); // enable end-of-operation interrupt
            w.psize().variant(0b11); // double-word parallelism
            w.ser().clear_bit(); // not sector erase
            w.pg().set_bit(); // programming
            w
        });
        for i in (0..Self::FLASH_ROW_SIZE).step_by(4) {
            let word = u32::from_le_bytes([
                self.buffer[src + i],
                self.buffer[src + i + 1],
                self.buffer[src + i + 2],
                self.buffer[src + i + 3],
            ]);
            let addr = (dest + i) as *mut u32;
            unsafe { addr.write_volatile(word) };
        }
        self.state = State::Programming { dest, src, len };
    }

    pub fn program(&mut self, offset: usize, data: &[u8]) -> nb::Result<(), Error> {
        if offset % Self::FLASH_ROW_SIZE != 0 {
            return Err(Error::NotAligned.into());
        }
        if data.len() % Self::FLASH_ROW_SIZE != 0 {
            return Err(Error::NotAligned.into());
        }
        if data.len() > self.buffer.len() {
            return Err(Error::OutOfBounds.into());
        }
        if offset + data.len() > Self::FLASH_BANK_SIZE {
            return Err(Error::OutOfBounds.into());
        }
        if let State::Idle { .. } = self.state {
            self.buffer[..data.len()].copy_from_slice(data);
            let dest = Self::FLASH_BANK2_BASE + offset;
            self.start_program_row(dest, 0, data.len());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

pub struct Iflash {
    inner: Mutex<RefCell<Inner>>,
}

impl Iflash {
    pub fn new(flash: FLASH, buffer: &'static mut [u8]) -> Self {
        let inner = Mutex::new(RefCell::new(Inner::new(flash, buffer)));
        Iflash { inner }
    }

    pub fn poll(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.poll();
        });
    }
}

impl IflashBind for Iflash {
    fn start_erase(&self) -> nb::Result<(), Infallible> {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.erase()
        })
    }

    fn start_program(&self, offset: usize, data: &[u8]) -> nb::Result<(), Error> {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.program(offset, data)
        })
    }

    fn status(&self) -> nb::Result<(), Error> {
        cortex_m::interrupt::free(|cs| {
            let inner = self.inner.borrow(cs).borrow();
            match inner.state {
                State::Idle { last_error: None } => Ok(()),
                State::Idle {
                    last_error: Some(last_error),
                } => Err(nb::Error::Other(last_error)),
                State::Erasing(_) | State::Programming { .. } => Err(nb::Error::WouldBlock),
            }
        })
    }
}
