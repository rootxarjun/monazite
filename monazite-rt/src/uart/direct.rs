mod traits;

use traits::{IdleInterrupt, RxSerial, TxSerial};

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use hal::stm32::usart1;
use stm32h7xx_hal as hal;

use ringbuf::RingBuf;

pub enum RxError {
    FifoOver,
}

pub trait Txfer {
    fn clear_interrupts(&mut self);
    fn restart(&mut self, f: &mut dyn FnMut(usize) -> &'static [u8]);
    fn enable(&mut self);
    fn disable(&mut self);
    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock;
}

impl<STREAM, PERIPHERAL> Txfer
    for hal::dma::Transfer<
        STREAM,
        PERIPHERAL,
        hal::dma::MemoryToPeripheral,
        &'static [u8],
        hal::dma::ConstDBTransfer,
    >
where
    STREAM: hal::dma::traits::Stream<
            Config = hal::dma::dma::DmaConfig,
            Interrupts = hal::dma::dma::DmaInterrupts,
        > + hal::dma::traits::DoubleBufferedStream,
    PERIPHERAL:
        hal::dma::traits::TargetAddress<hal::dma::MemoryToPeripheral, MemSize = u8> + TxSerial,
{
    fn clear_interrupts(&mut self) {
        Self::clear_interrupts(self);
    }

    fn restart(&mut self, f: &mut dyn FnMut(usize) -> &'static [u8]) {
        Self::next_transfer_with(self, |old_buf, _, remaining| {
            let complete_len = old_buf.len() - remaining;
            let new_buf = f(complete_len);
            (new_buf, ())
        })
        .expect("BUG: DMA transfer failed");
    }

    fn enable(&mut self) {
        self.start(|tx| {
            tx.enable_dma_tx();
        });
    }

    fn disable(&mut self) {
        self.pause(|tx| {
            tx.disable_dma_tx();
        });
    }

    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock {
        PERIPHERAL::inner()
    }
}

pub trait Rxfer {
    fn clear_interrupts(&mut self);
    fn restart(&mut self, f: &mut dyn FnMut(usize) -> &'static mut [u8]);
    fn enable(&mut self);
    fn disable(&mut self);
    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock;
}

impl<STREAM, PERIPHERAL> Rxfer
    for hal::dma::Transfer<
        STREAM,
        PERIPHERAL,
        hal::dma::PeripheralToMemory,
        &'static mut [u8],
        hal::dma::DBTransfer,
    >
where
    STREAM: hal::dma::traits::Stream<
            Config = hal::dma::dma::DmaConfig,
            Interrupts = hal::dma::dma::DmaInterrupts,
        > + hal::dma::traits::DoubleBufferedStream,
    PERIPHERAL: hal::dma::traits::TargetAddress<hal::dma::PeripheralToMemory, MemSize = u8>
        + IdleInterrupt
        + RxSerial,
{
    fn clear_interrupts(&mut self) {
        Self::clear_interrupts(self);
    }

    fn restart(&mut self, f: &mut dyn FnMut(usize) -> &'static mut [u8]) {
        Self::next_transfer_with(self, |old_buf, _, remaining| {
            let complete_len = old_buf.len() - remaining;
            let new_buf = f(complete_len);
            (new_buf, ())
        })
        .expect("BUG: DMA transfer failed");
    }

    fn enable(&mut self) {
        self.start(|rx| {
            rx.enable_dma_rx();
        });
    }

    fn disable(&mut self) {
        self.pause(|rx| {
            rx.disable_dma_rx();
        });
    }

    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock {
        PERIPHERAL::inner()
    }
}

struct TxInner {
    ring: RingBuf<'static>,
    txfer: &'static mut dyn Txfer,
}

unsafe impl Send for TxInner {}

impl TxInner {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> bool {
        self.restart();

        if self.ring.available() < buf.len() {
            return false;
        }
        let send_len = self.ring.write(buf);
        self.ring.complete_write(send_len);

        self.restart();
        true
    }

    #[inline]
    fn complete_read(&mut self) {
        self.restart();
    }

    #[inline]
    fn restart(&mut self) {
        self.txfer.restart(&mut |complete_len| {
            self.ring.complete_read(complete_len);
            let (first, _) = self.ring.readable();
            let new_buf = unsafe { core::slice::from_raw_parts(first.as_ptr(), first.len()) };
            new_buf
        });
    }

    #[inline]
    fn reset(&mut self) {
        self.ring.clear();
        self.txfer.clear_interrupts();
        self.restart();
    }

    #[inline]
    fn enable(&mut self) {
        self.txfer.enable();
    }

    #[inline]
    fn disable(&mut self) {
        self.txfer.disable();
    }

    #[inline]
    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock {
        self.txfer.inner()
    }
}

pub struct Tx {
    inner: Mutex<RefCell<TxInner>>,
    kernel_clock: u32,
}

impl Tx {
    pub fn new(
        buffer: &'static mut [u8],
        txfer: &'static mut dyn Txfer,
        kernel_clock: u32,
    ) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(TxInner {
                ring: RingBuf::new(buffer),
                txfer,
            })),
            kernel_clock,
        }
    }

    pub fn write(&self, buf: &[u8]) -> bool {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.write(buf)
        })
    }

    pub fn readable<T>(&self, f: impl FnOnce(&[u8], &[u8]) -> T) -> T {
        cortex_m::interrupt::free(|cs| {
            let inner = self.inner.borrow(cs).borrow();
            let (first, second) = inner.ring.readable();
            f(first, second)
        })
    }

    #[allow(clippy::result_unit_err)]
    pub fn complete_read(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.complete_read();
        });
    }

    pub fn enable(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.enable();
        });
    }

    pub fn disable(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.disable();
        });
    }

    pub fn reset(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.reset();
        });
    }

    pub unsafe fn inner(&self) -> &'static usart1::RegisterBlock {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.inner()
        })
    }
}

struct RxInner {
    ring: RingBuf<'static>,
    rxfer: &'static mut dyn Rxfer,
    need_to_return_error: bool,
}

unsafe impl Send for RxInner {}

impl RxInner {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, RxError> {
        if self.need_to_return_error {
            self.need_to_return_error = false;
            return Err(RxError::FifoOver);
        }

        self.restart();

        if self.ring.is_full() {
            self.need_to_return_error = true;
        }

        let read_len = self.ring.read(buf);
        self.ring.complete_read(read_len);

        self.restart();

        Ok(read_len)
    }

    #[allow(clippy::result_unit_err)]
    pub fn complete_write(&mut self) {
        self.restart();
    }

    #[inline]
    fn restart(&mut self) {
        self.rxfer.restart(&mut |complete_len| {
            self.ring.complete_write(complete_len);
            let (first, _) = self.ring.writable();
            let new_buf =
                unsafe { core::slice::from_raw_parts_mut(first.as_mut_ptr(), first.len()) };
            new_buf
        });
    }

    #[inline]
    fn reset(&mut self) {
        self.ring.clear();
        self.need_to_return_error = false;
        self.rxfer.clear_interrupts();
        self.restart();
    }

    #[inline]
    fn enable(&mut self) {
        self.rxfer.enable();
    }

    #[inline]
    fn disable(&mut self) {
        self.rxfer.disable();
    }

    #[inline]
    unsafe fn inner(&mut self) -> &'static usart1::RegisterBlock {
        self.rxfer.inner()
    }
}

pub struct Rx {
    inner: Mutex<RefCell<RxInner>>,
}

impl Rx {
    pub fn new(buffer: &'static mut [u8], rxfer: &'static mut dyn Rxfer) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(RxInner {
                ring: RingBuf::new(buffer),
                rxfer,
                need_to_return_error: false,
            })),
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, RxError> {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.read(buf)
        })
    }

    pub fn writeable<T>(&self, f: impl FnOnce(&mut [u8], &mut [u8]) -> T) -> T {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            let (first, second) = inner.ring.writable();
            f(first, second)
        })
    }

    #[allow(clippy::result_unit_err)]
    pub fn complete_write(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.complete_write();
        });
    }

    pub fn reset(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.reset();
        });
    }

    pub fn enable(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.enable();
        });
    }

    pub fn disable(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.disable();
        });
    }

    pub unsafe fn inner(&self) -> &'static usart1::RegisterBlock {
        cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();
            inner.inner()
        })
    }
}

pub fn serial_inner<'a>(tx: &'a Tx, rx: &'a Rx) -> Option<&'a usart1::RegisterBlock> {
    let inner = unsafe {
        use core::ptr;
        let inner = tx.inner();
        if !ptr::eq(ptr::from_ref(tx.inner()), ptr::from_ref(rx.inner())) {
            return None;
        }
        inner
    };
    Some(inner)
}

pub fn set_baud_rate(baud_rate: u32, tx: &Tx, rx: &Rx) {
    tx.disable();
    rx.disable();

    if let Some(serial) = serial_inner(tx, rx) {
        // RM0433 spec p.2076
        serial.cr1.modify(|_, w| w.te().clear_bit());
        while serial.isr.read().tc().bit_is_clear() {} // wait for isr.tc to be set
        serial.cr1.modify(|_, w| w.ue().clear_bit());

        let ker_clk = tx.kernel_clock;
        let brr_value = ker_clk / baud_rate;
        serial.brr.write(|w| {
            w.brr()
                .variant(u16::try_from(brr_value).expect("brr value over flow"))
        });

        serial.cr1.modify(|_, w| w.ue().set_bit());
        serial.cr1.modify(|_, w| w.te().set_bit());
    }

    tx.enable();
    rx.enable();
}

pub fn reset(tx: &Tx, rx: &Rx) {
    tx.reset();
    rx.reset();
}
