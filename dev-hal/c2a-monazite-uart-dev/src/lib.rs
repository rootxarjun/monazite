mod buffer;
mod kble;

use std::net::SocketAddr;
use std::{collections::HashMap, sync::Arc};

use c2a_monazite_uart_bind::{ChannelId, Error as UartError, Uart as UartBind};
use kble::Server;
use tokio::sync::{Mutex, OwnedMutexGuard, RwLock};

use buffer::Buffer;

pub struct OuterChannel {
    pub tx: Arc<Buffer>,
    pub rx: Arc<Buffer>,
}

struct InnerChannel {
    tx: Arc<Buffer>,
    rx: Arc<Buffer>,
}

struct ChannelPair {
    inner: InnerChannel,
    outer: Arc<Mutex<OuterChannel>>,
}

impl ChannelPair {
    fn with_capacity(capacity: usize) -> Self {
        let tx = Arc::new(Buffer::with_capacity(capacity));
        let rx = Arc::new(Buffer::with_capacity(capacity));
        let inner = InnerChannel {
            tx: tx.clone(),
            rx: rx.clone(),
        };
        let outer = OuterChannel { tx, rx };

        Self {
            inner,
            outer: Arc::new(Mutex::new(outer)),
        }
    }

    fn reinitialize(&mut self, capacity: usize) {
        self.inner.tx.reinitialize(capacity);
        self.inner.rx.reinitialize(capacity);
    }
}

#[derive(Default)]
pub struct Mux {
    channels: RwLock<HashMap<u8, ChannelPair>>,
}

impl Mux {
    fn init_channel(&self, ch: u8) {
        const BUFFER_SIZE: usize = 2048; // FIXME: make configurable
        let mut channels = self.channels.blocking_write();
        channels
            .entry(ch)
            .and_modify(|channel| channel.reinitialize(BUFFER_SIZE))
            .or_insert_with(|| ChannelPair::with_capacity(BUFFER_SIZE));
    }

    fn receive(&self, ch: u8, buf: &mut [u8]) -> Result<usize, UartError> {
        let channels = self.channels.blocking_read();
        let Some(pair) = channels.get(&ch) else {
            return Err(UartError::Channel);
        };
        Ok(pair.inner.rx.nonblocking_read(buf))
    }

    fn send(&self, ch: u8, data: &[u8]) -> Result<(), UartError> {
        let channels = self.channels.blocking_read();
        let Some(pair) = channels.get(&ch) else {
            return Err(UartError::Channel);
        };
        pair.inner.tx.blocking_write(data);
        Ok(())
    }

    pub fn try_get_outer(&self, ch: u8) -> Option<OwnedMutexGuard<OuterChannel>> {
        let channels = self.channels.try_read().ok()?;
        let pair = channels.get(&ch)?;
        pair.outer.clone().try_lock_owned().ok()
    }
}

pub struct Uart {
    mux: Arc<Mux>,
}

impl Uart {
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        let mux = Arc::new(Mux::default());
        let server = Server::new(mux.clone());
        server.serve_in_background(addr);
        Self { mux }
    }
}

impl UartBind for Uart {
    fn initialize(&self, ch: ChannelId, _baudrate: u32) -> Result<(), UartError> {
        self.mux.init_channel(ch.into());
        Ok(())
    }

    fn reopen(&self, ch: ChannelId, _baudrate: u32) -> Result<(), UartError> {
        self.mux.init_channel(ch.into());
        Ok(())
    }

    fn send(&self, ch: ChannelId, data: &[u8]) -> Result<(), UartError> {
        self.mux.send(ch.into(), data)
    }

    fn receive(&self, ch: ChannelId, buffer: &mut [u8]) -> Result<usize, UartError> {
        self.mux.receive(ch.into(), buffer)
    }
}
