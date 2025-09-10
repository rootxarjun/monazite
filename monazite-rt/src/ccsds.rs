use c2a_monazite_ccsds_bind::{Ccsds as CcsdsBind, Error, RxStats};

pub struct Ccsds;

impl Ccsds {
    pub fn new() -> Self {
        Self
    }
}

impl CcsdsBind for Ccsds {
    fn initialize(&self) -> Result<(), Error> {
        Ok(())
    }

    fn reopen(&self) -> Result<(), Error> {
        Ok(())
    }

    fn send(&self, _data: &[u8]) -> Result<(), Error> {
        Ok(())
    }

    fn receive(&self, _buffer: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    fn tx_buffer_free_frames(&self) -> usize {
        1
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
        // noop
    }
}
