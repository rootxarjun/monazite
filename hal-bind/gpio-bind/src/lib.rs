#![no_std]

mod bind;

use core::ffi::{c_int, c_uchar, c_void};

use atomic_once_cell::AtomicOnceCell;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[non_exhaustive]
pub enum Error {
    Unknown = bind::GPIO_ERR_CODE_GPIO_UNKNOWN_ERR.0,
    Port = bind::GPIO_ERR_CODE_GPIO_PORT_ERR.0,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Value {
    Low = bind::GPIO_HL_GPIO_LOW.0,
    High = bind::GPIO_HL_GPIO_HIGH.0,
}

impl TryFrom<u8> for Value {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Low),
            1 => Ok(Self::High),
            _ => Err(()),
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Low => false,
            Value::High => true,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        if value {
            Value::High
        } else {
            Value::Low
        }
    }
}

pub trait Gpio: Sync {
    /// GPIO のハードウェアを初期化する
    /// # Errors
    /// ハードウェアの初期化に失敗した場合は [`Error`] が返る。
    fn initialize(&self) -> Result<(), Error>;

    /// `port` で指定された出力ポートに `output` の値を出力する
    /// # Errors
    /// 出力ポートが存在しない場合や、出力に失敗した場合は [`Error`] が返る。
    fn set_output(&self, port: u8, output: Value) -> Result<(), Error>;

    /// `port` で指定された出力ポートが出力している値を取得する
    /// # Errors
    /// 出力ポートが存在しない場合や、出力状態の読み出しに失敗した場合は [`Error`] が返る。
    fn get_output(&self, port: u8) -> Result<Value, Error>;

    /// `port` で指定された入力ポートの値を取得する
    /// # Errors
    /// 入力ポートが存在しない場合や、入力状態の読み出しに失敗した場合は [`Error`] が返る。
    fn get_input(&self, port: u8) -> Result<Value, Error>;
}

#[no_mangle]
pub static C2A_MONAZITE_GPIO: AtomicOnceCell<&'static dyn Gpio> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn GPIO_initialize(_: *mut c_void) -> c_int {
    let gpio = C2A_MONAZITE_GPIO.get();
    match gpio.initialize() {
        Ok(()) => bind::GPIO_ERR_CODE_GPIO_OK.0,
        Err(err) => err as i32,
    }
}

#[no_mangle]
pub extern "C" fn GPIO_set_output(port_id: c_uchar, value: c_uchar) -> c_int {
    let gpio = C2A_MONAZITE_GPIO.get();
    let Ok(value) = Value::try_from(value) else {
        return bind::GPIO_ERR_CODE_GPIO_LOGIC_ERR.0;
    };
    match gpio.set_output(port_id, value) {
        Ok(()) => bind::GPIO_ERR_CODE_GPIO_OK.0,
        Err(err) => err as i32,
    }
}

#[no_mangle]
pub extern "C" fn GPIO_get_output(port_id: c_uchar) -> c_int {
    let gpio = C2A_MONAZITE_GPIO.get();
    match gpio.get_output(port_id) {
        Ok(value) => value as i32,
        Err(err) => err as i32,
    }
}

#[no_mangle]
pub extern "C" fn GPIO_get_input(port_id: c_uchar) -> c_int {
    let gpio = C2A_MONAZITE_GPIO.get();
    match gpio.get_input(port_id) {
        Ok(value) => value as i32,
        Err(err) => err as i32,
    }
}
