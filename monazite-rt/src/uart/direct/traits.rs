use hal::stm32::{usart1, UART4, UART5, UART7, USART1, USART2, USART3};
use stm32h7xx_hal as hal;

pub trait TxSerial {
    fn enable_dma_tx(&mut self);
    fn disable_dma_tx(&mut self);
    unsafe fn inner() -> &'static usart1::RegisterBlock;
}

impl TxSerial for hal::serial::Tx<hal::pac::USART1> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART1::PTR }
    }
}

impl TxSerial for hal::serial::Tx<hal::pac::USART2> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART2::PTR }
    }
}

impl TxSerial for hal::serial::Tx<hal::pac::USART3> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART3::PTR }
    }
}

impl TxSerial for hal::serial::Tx<hal::pac::UART4> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART4::PTR }
    }
}

impl TxSerial for hal::serial::Tx<hal::pac::UART5> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART5::PTR }
    }
}

impl TxSerial for hal::serial::Tx<hal::pac::UART7> {
    #[inline]
    fn enable_dma_tx(&mut self) {
        Self::enable_dma_tx(self);
    }
    #[inline]
    fn disable_dma_tx(&mut self) {
        Self::disable_dma_tx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART7::PTR }
    }
}

pub trait RxSerial {
    fn enable_dma_rx(&mut self);
    fn disable_dma_rx(&mut self);
    unsafe fn inner() -> &'static usart1::RegisterBlock;
}

impl RxSerial for hal::serial::Rx<hal::pac::USART1> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART1::PTR }
    }
}

impl RxSerial for hal::serial::Rx<hal::pac::USART2> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART2::PTR }
    }
}

impl RxSerial for hal::serial::Rx<hal::pac::USART3> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*USART3::PTR }
    }
}

impl RxSerial for hal::serial::Rx<hal::pac::UART4> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART4::PTR }
    }
}

impl RxSerial for hal::serial::Rx<hal::pac::UART5> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART5::PTR }
    }
}

impl RxSerial for hal::serial::Rx<hal::pac::UART7> {
    #[inline]
    fn enable_dma_rx(&mut self) {
        Self::enable_dma_rx(self);
    }
    #[inline]
    fn disable_dma_rx(&mut self) {
        Self::disable_dma_rx(self);
    }
    #[inline]
    unsafe fn inner() -> &'static usart1::RegisterBlock {
        unsafe { &*UART7::PTR }
    }
}

pub trait IdleInterrupt {
    fn clear_idle(&mut self);
}

impl IdleInterrupt for hal::serial::Rx<hal::pac::USART1> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
impl IdleInterrupt for hal::serial::Rx<hal::pac::USART2> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
impl IdleInterrupt for hal::serial::Rx<hal::pac::USART3> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
impl IdleInterrupt for hal::serial::Rx<hal::pac::UART4> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
impl IdleInterrupt for hal::serial::Rx<hal::pac::UART5> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
impl IdleInterrupt for hal::serial::Rx<hal::pac::UART7> {
    #[inline]
    fn clear_idle(&mut self) {
        Self::clear_idle(self);
    }
}
