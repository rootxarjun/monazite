pub mod direct;

use c2a_monazite_uart_bind::{ChannelId, Error, Uart as UartBind};

pub const DIRECT_UART_NUM: usize = 6;
pub type DirectUartArray = [(direct::Tx, direct::Rx); DIRECT_UART_NUM];

pub struct Uart {
    direct: &'static DirectUartArray,
}

impl Uart {
    pub fn new(direct: &'static DirectUartArray) -> Self {
        Self { direct }
    }
}

impl UartBind for Uart {
    fn initialize(&self, ch: ChannelId, baudrate: u32) -> Result<(), Error> {
        let ch = u8::from(ch) as usize;
        if let Some((tx, rx)) = self.direct.get(ch) {
            direct::set_baud_rate(baudrate, tx, rx);
            direct::reset(tx, rx);
            Ok(())
        } else {
            Err(Error::Channel)
        }
    }

    fn reopen(&self, ch: ChannelId, baudrate: u32) -> Result<(), Error> {
        self.initialize(ch, baudrate)
    }

    fn send(&self, ch: ChannelId, data: &[u8]) -> Result<(), Error> {
        let ch = u8::from(ch) as usize;
        if let Some((tx, _rx)) = self.direct.get(ch) {
            defmt::trace!("UART_tx(direct): ch: {} write {} bytes", ch, data.len());
            if tx.write(data) {
                Ok(())
            } else {
                Err(Error::FifoFull)
            }
        } else {
            Err(Error::Channel)
        }
    }

    fn receive(&self, ch: ChannelId, buffer: &mut [u8]) -> Result<usize, Error> {
        let ch = u8::from(ch) as usize;
        if let Some((_tx, rx)) = self.direct.get(ch) {
            match rx.read(buffer) {
                Ok(read_len) => {
                    defmt::trace!("UART_rx(direct): ch: {} read {} bytes", ch, read_len);
                    Ok(read_len)
                }
                Err(direct::RxError::FifoOver) => Err(Error::FifoOverrun),
            }
        } else {
            Err(Error::Channel)
        }
    }
}
