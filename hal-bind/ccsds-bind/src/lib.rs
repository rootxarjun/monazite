#![no_std]

mod bind;

use core::ffi::{c_int, c_uchar, c_void};

use atomic_once_cell::AtomicOnceCell;

pub use bind::CCSDS_RxStats as RxStats;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Error {
    TxNoBuffer = bind::CCSDS_ERR_CODE_CCSDS_ERR_TX_NO_BUFFER.0,
    TxInvalid = bind::CCSDS_ERR_CODE_CCSDS_ERR_TX_INVALID.0,
    TxSizeError = bind::CCSDS_ERR_CODE_CCSDS_ERR_TX_SIZE_ERR.0,
    Rx4Kbps = bind::CCSDS_ERR_CODE_CCSDS_ERR_RX_4KBPS.0,
    Rx1Kbps = bind::CCSDS_ERR_CODE_CCSDS_ERR_RX_1KBPS.0,
    ParameterError = bind::CCSDS_ERR_CODE_CCSDS_ERR_PARAM_ERR.0,
}

fn result_to_error_code_int(result: Result<(), Error>) -> c_int {
    match result {
        Ok(()) => bind::CCSDS_ERR_CODE_CCSDS_ERR_OK.0,
        Err(err) => err as c_int,
    }
}

pub trait Ccsds: Sync {
    /// Sバンド送受信機ドライバのハードウェアを初期化する
    /// # Errors
    /// ハードウェアの初期化に失敗した場合は [`Error`] が返る。
    fn initialize(&self) -> Result<(), Error>;

    /// # Errors
    /// 再オープンに失敗した場合は [`Error`] が返る。
    fn reopen(&self) -> Result<(), Error>;

    /// # Errors
    /// 送信に失敗した場合は [`Error`] が返る。
    /// 特に、送信バッファに空きがない場合は [`Error::TxNoBuffer`] が返る。
    fn send(&self, data: &[u8]) -> Result<(), Error>;

    /// # Errors
    /// 受信に失敗した場合は [`Error`] が返る。
    fn receive(&self, buffer: &mut [u8]) -> Result<usize, Error>;

    fn tx_buffer_free_frames(&self) -> usize;

    fn rx_stats(&self) -> RxStats;

    fn set_aos_scid(&self, aos_scid: u8);
}

#[no_mangle]
pub static C2A_MONAZITE_CCSDS: AtomicOnceCell<&'static dyn Ccsds> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn CCSDS_init(_ccsds_v: *mut c_void) -> c_int {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    result_to_error_code_int(ccsds.initialize())
}

#[no_mangle]
pub extern "C" fn CCSDS_reopen(_ccsds_v: *mut c_void, _reason: c_int) -> c_int {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    result_to_error_code_int(ccsds.reopen())
}

/// # Safety
/// `data_v` は `data_size` バイトのメモリ領域を指している必要がある。
#[no_mangle]
pub unsafe extern "C" fn CCSDS_tx(
    _ccsds_v: *mut c_void,
    data_v: *mut c_void,
    data_size: c_int,
) -> c_int {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    if data_size < 0 {
        return Error::ParameterError as c_int;
    }
    // Safety: data_size は非負であることを検査済み
    #[allow(clippy::cast_sign_loss)]
    let data = core::slice::from_raw_parts(data_v.cast::<c_uchar>(), data_size as usize);
    result_to_error_code_int(ccsds.send(data))
}

/// # Safety
/// `data_v` は `buffer_size` バイトのメモリ領域を指している必要がある。
#[no_mangle]
pub unsafe extern "C" fn CCSDS_rx(
    _ccsds_v: *mut c_void,
    data_v: *mut c_void,
    buffer_size: c_int,
) -> c_int {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    if buffer_size < 0 {
        return Error::ParameterError as c_int;
    }
    // Safety: buffer_size は非負であることを検査済み
    #[allow(clippy::cast_sign_loss)]
    let buffer = core::slice::from_raw_parts_mut(data_v.cast::<c_uchar>(), buffer_size as usize);
    match ccsds.receive(buffer) {
        // Safety: 受信バッファのサイズは十分に小さいため桁あふれは発生しない
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        Ok(len) => len as c_int,
        Err(err) => err as c_int,
    }
}

#[no_mangle]
pub extern "C" fn CCSDS_get_buffer_num() -> u8 {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    u8::try_from(ccsds.tx_buffer_free_frames()).unwrap_or(u8::MAX)
}

/// # Safety
/// `rx_stats` は有効なメモリ領域を指している必要がある。
#[no_mangle]
pub unsafe extern "C" fn CCSDS_get_rx_stats(rx_stats: *mut RxStats) {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    *rx_stats = ccsds.rx_stats();
}

#[no_mangle]
pub extern "C" fn CCSDS_set_aos_scid(aos_scid: u8) {
    let ccsds = C2A_MONAZITE_CCSDS.get();
    ccsds.set_aos_scid(aos_scid);
}

#[no_mangle]
pub extern "C" fn CCSDS_set_rate(_ui_rate: u32, _ccsds_config: *mut c_void) {
    // no-op
}
