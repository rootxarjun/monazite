macro_rules! direct_uart {
    (
        $res:expr, $shared:expr, $tx_buffers:expr, $rx_buffers:expr,
        {$(
            $logical_ch:tt: $USARTX:ident,
        )+}
    ) => {
        [
            $({
                use hal::dma::dma::StreamX;
                use hal::pac::{DMA1, DMA2};

                type Txfer<STREAM, USART> =
                    Transfer<STREAM, hal::serial::Tx<USART>, MemoryToPeripheral, &'static [u8], ConstDBTransfer>;
                type Rxfer<STREAM, USART> =
                    Transfer<STREAM, hal::serial::Rx<USART>, PeripheralToMemory, &'static mut [u8], DBTransfer>;

                let ch = $res.channels.$logical_ch;
                let pins = ($res.pins.$logical_ch.tx.into_alternate(), $res.pins.$logical_ch.rx.into_alternate());
                let serial = ch.uart
                    .serial(pins, (115200).bps(), ch.rec, &$shared.clocks)
                    .unwrap();
                let kernel_clk = hal::serial::Serial::<hal::pac::$USARTX>::kernel_clk_unwrap(&$shared.clocks).raw();
                let (mut tx, mut rx) = serial.split();

                let dma_tx = ch.dma_tx;
                let dma_rx = ch.dma_rx;

                let txfer = {
                    let config = DmaConfig::default()
                        .memory_increment(true)
                        .transfer_complete_interrupt(true)
                        .fifo_enable(true);
                    tx.enable_dma_tx();
                    cortex_m::singleton!(: Txfer<StreamX<DMA1, $logical_ch>, hal::pac::$USARTX> = Transfer::init_const(dma_tx, tx, &[][..], None, config)).unwrap()
                } as &'static mut dyn crate::uart::direct::Txfer;

                let rxfer = {
                    let config = DmaConfig::default()
                        .memory_increment(true)
                        .transfer_complete_interrupt(true)
                        .fifo_enable(false);
                    rx.enable_dma_rx();
                    cortex_m::singleton!(: Rxfer<StreamX<DMA2, $logical_ch>, hal::pac::$USARTX> = Transfer::init(dma_rx, rx, &mut [][..], None, config)).unwrap()
                } as &'static mut dyn crate::uart::direct::Rxfer;

                unsafe {(
                    crate::uart::direct::Tx::new(
                        $tx_buffers[$logical_ch].assume_init_mut(),
                        txfer,
                        kernel_clk,
                    ),
                    crate::uart::direct::Rx::new(
                        $rx_buffers[$logical_ch].assume_init_mut(),
                        rxfer,
                    ),
                )}
            },)+
        ]
    };
}
