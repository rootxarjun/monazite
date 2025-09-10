#![no_std]

use core::ffi::{c_int, c_void};

use atomic_once_cell::AtomicOnceCell;
use c2a_core::hal::uart as bind;

pub const CHANNEL_NUM: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Error {
    Unknown = bind::UART_ERR_CODE_UART_UNKNOWN_ERR.0,
    DataNegative = bind::UART_ERR_CODE_UART_DATA_NEGA_ERR.0,
    FifoFull = bind::UART_ERR_CODE_UART_FIFO_FULL_ERR.0,
    RxAll = bind::UART_ERR_CODE_UART_RX_ALL_ERR.0,
    FifoStop = bind::UART_ERR_CODE_UART_FIFO_STOP_ERR.0,
    ParityStop = bind::UART_ERR_CODE_UART_PARITY_STOP_ERR.0,
    StopBit = bind::UART_ERR_CODE_UART_STOP_BIT_ERR.0,
    ParityFifo = bind::UART_ERR_CODE_UART_PARITY_FIFO_ERR.0,
    FifoOverrun = bind::UART_ERR_CODE_UART_FIFO_OVER_ERR.0,
    Parity = bind::UART_ERR_CODE_UART_PARITY_ERR.0,
    NotYet = bind::UART_ERR_CODE_UART_YET_ERR.0,
    Already = bind::UART_ERR_CODE_UART_ALREADY_ERR.0,
    Baudrate = bind::UART_ERR_CODE_UART_BAUDRATE_ERR.0,
    Channel = bind::UART_ERR_CODE_UART_CH_ERR.0,
}

fn result_to_error_code_int(result: Result<(), Error>) -> c_int {
    match result {
        Ok(()) => bind::UART_ERR_CODE_UART_OK.0,
        Err(err) => err as c_int,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChannelId(u8);

impl From<ChannelId> for u8 {
    fn from(value: ChannelId) -> Self {
        value.0
    }
}

impl TryFrom<u8> for ChannelId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Safety: `CHANNEL_NUM` は必ず `u8` に収まる
        #[allow(clippy::cast_possible_truncation)]
        if value < CHANNEL_NUM as u8 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

pub trait Uart: Sync {
    /// # Errors
    /// 初期化に失敗した場合は [`Error`] が返る。
    fn initialize(&self, ch: ChannelId, baudrate: u32) -> Result<(), Error>;

    /// # Errors
    /// 再オープンに失敗した場合は [`Error`] が返る。
    fn reopen(&self, ch: ChannelId, baudrate: u32) -> Result<(), Error>;

    /// # Errors
    /// 送信に失敗した場合は [`Error`] が返る。
    /// 特に、送信バッファに空きがない場合は [`Error::FifoFull`] が返る。
    fn send(&self, ch: ChannelId, data: &[u8]) -> Result<(), Error>;

    /// # Errors
    /// 受信に失敗した場合は [`Error`] が返る。
    fn receive(&self, ch: ChannelId, buffer: &mut [u8]) -> Result<usize, Error>;
}

#[no_mangle]
pub static C2A_MONAZITE_UART: AtomicOnceCell<&'static dyn Uart> = AtomicOnceCell::new();

/// # Safety
/// `config` は有効なポインタでなければならない。
#[no_mangle]
pub unsafe extern "C" fn UART_init(config: *mut bind::UART_Config) -> c_int {
    let uart = C2A_MONAZITE_UART.get();
    let config = &*config;
    let Ok(ch_id) = ChannelId::try_from(config.ch) else {
        return Error::Channel as c_int;
    };
    result_to_error_code_int(uart.initialize(ch_id, config.baudrate))
}

/// # Safety
/// `config` は有効なポインタでなければならない。
#[no_mangle]
pub unsafe extern "C" fn UART_reopen(config: *mut bind::UART_Config, _reason: c_int) -> c_int {
    let uart = C2A_MONAZITE_UART.get();
    let config = &*config;
    let Ok(ch_id) = ChannelId::try_from(config.ch) else {
        return Error::Channel as c_int;
    };
    result_to_error_code_int(uart.reopen(ch_id, config.baudrate))
}

/// # Safety
/// `config` は有効なポインタでなければならない。
/// また、`data_v` は `data_size` バイトのデータを指すポインタでなければならない。
#[no_mangle]
pub unsafe extern "C" fn UART_tx(
    config: *mut bind::UART_Config,
    data_v: *mut c_void,
    data_size: c_int,
) -> c_int {
    let uart = C2A_MONAZITE_UART.get();
    let config = &*config;
    if data_size < 0 {
        return Error::DataNegative as c_int;
    }
    // Safety: 事前に検査しているので `data_size` は必ず非負
    #[allow(clippy::cast_sign_loss)]
    let data = core::slice::from_raw_parts(data_v as *const u8, data_size as usize);
    let Ok(ch_id) = ChannelId::try_from(config.ch) else {
        return Error::Channel as c_int;
    };
    result_to_error_code_int(uart.send(ch_id, data))
}

/// # Safety
/// `config` は有効なポインタでなければならない。
/// また、`data_v` は `buffer_size` バイトのデータを格納する領域を指すポインタでなければならず、
/// 他に同時に読み書きできるエイリアスが存在してはならない。
#[no_mangle]
pub unsafe extern "C" fn UART_rx(
    config: *mut bind::UART_Config,
    data_v: *mut c_void,
    buffer_size: c_int,
) -> c_int {
    let uart = C2A_MONAZITE_UART.get();
    let config = &*config;
    if buffer_size < 0 {
        return Error::DataNegative as c_int;
    }
    // Safety: 事前に検査しているので `buffer_size` は必ず非負
    #[allow(clippy::cast_sign_loss)]
    let buffer = core::slice::from_raw_parts_mut(data_v.cast::<u8>(), buffer_size as usize);
    let Ok(ch_id) = ChannelId::try_from(config.ch) else {
        return Error::Channel as c_int;
    };
    match uart.receive(ch_id, buffer) {
        // Safety: 受信バッファは十分に小さいため、`size` は `c_int` に収まると仮定してよい
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        Ok(size) => size as c_int,
        Err(err) => err as c_int,
    }
}
