use core::cell::RefCell;

use c2a_monazite_wdt_bind::Wdt as WdtBind;
use cortex_m::interrupt::Mutex;
use hal::{independent_watchdog::IndependentWatchdog, pac, prelude::*};
use stm32h7xx_hal as hal;

pub struct Wdt {
    inner: Mutex<RefCell<IndependentWatchdog>>,
}

impl Wdt {
    pub fn new(iwdg: pac::IWDG) -> Self {
        let iwdg = IndependentWatchdog::new(iwdg);
        Self {
            inner: Mutex::new(RefCell::new(iwdg)),
        }
    }
}

impl WdtBind for Wdt {
    fn initialize(&self) {
        // no-op
    }

    fn clear(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.feed();
        });
    }

    fn enable(&self, time: u32) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.start(time.millis());
        });
    }
}
