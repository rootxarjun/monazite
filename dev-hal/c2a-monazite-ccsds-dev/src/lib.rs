mod kble;

use std::{net::SocketAddr, sync::Mutex};

use c2a_monazite_ccsds_bind::{Ccsds as CcsdsBind, Error, RxStats};
use tokio::sync::mpsc::{self, error::TryRecvError};

pub struct Ccsds {
    tlm_tx: mpsc::Sender<Vec<u8>>,
    cmd_rx: Mutex<mpsc::Receiver<Vec<u8>>>,
}

impl Ccsds {
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        let (tlm_tx, cmd_rx, socket) = kble::new();
        let cmd_rx = Mutex::new(cmd_rx);
        socket.serve_in_background(addr);
        Self { tlm_tx, cmd_rx }
    }
}

impl CcsdsBind for Ccsds {
    fn initialize(&self) -> Result<(), Error> {
        Ok(())
    }

    fn reopen(&self) -> Result<(), Error> {
        Ok(())
    }

    fn send(&self, data: &[u8]) -> Result<(), Error> {
        self.tlm_tx
            .try_send(data.to_vec())
            .map_err(|_| Error::TxNoBuffer)
    }

    fn receive(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let cmd_bytes = match self.cmd_rx.lock().unwrap().try_recv() {
            Ok(cmd_bytes) => cmd_bytes,
            Err(TryRecvError::Empty) => return Ok(0),
            _ => return Err(Error::Rx4Kbps),
        };
        if cmd_bytes.len() > buffer.len() {
            // buffer is too short
            return Err(Error::Rx4Kbps);
        }
        let len = cmd_bytes.len();
        buffer[..len].copy_from_slice(&cmd_bytes[..]);
        Ok(len)
    }

    fn tx_buffer_free_frames(&self) -> usize {
        8
    }

    fn rx_stats(&self) -> RxStats {
        RxStats {
            corrupted_frames: 0,
            overflowed_frames: 0,
            found_starts: 0,
            skipped_frames: 0,
            last_frame_corrected_errors: 0,
        }
    }

    fn set_aos_scid(&self, _aos_scid: u8) {
        // no-op
    }
}
