#![no_std]

mod bind;

use core::ffi::c_int;

use atomic_once_cell::AtomicOnceCell;

pub const TEST_CHANNEL_NUM: usize = 2;
pub const INPUT_CHANNEL_NUM: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InputChannelId(u8);

impl From<InputChannelId> for u8 {
    fn from(value: InputChannelId) -> Self {
        value.0
    }
}

impl TryFrom<u8> for InputChannelId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Safety: `INPUT_CHANNEL_NUM` は必ず `u8` に収まる
        #[allow(clippy::cast_possible_truncation)]
        if value < INPUT_CHANNEL_NUM as u8 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TestChannelId(u8);

impl From<TestChannelId> for u8 {
    fn from(value: TestChannelId) -> Self {
        value.0
    }
}

impl TryFrom<u8> for TestChannelId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Safety: `TEST_CHANNEL_NUM` は必ず `u8` に収まる
        #[allow(clippy::cast_possible_truncation)]
        if value < TEST_CHANNEL_NUM as u8 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[non_exhaustive]
pub enum Error {
    ChannelError = bind::ADC_ERR_CODE_ADC_CHANNEL_ERR.0,
}

pub trait Adc: Sync {
    /// ADC の初期化を行う
    /// # Errors
    /// ハードウェアの初期化に失敗した場合、[`Error`] を返す。
    fn initialize(&self) -> Result<(), Error>;
    /// 指定した入力チャンネルの値を取得する
    fn get_value(&self, ch: InputChannelId) -> u16;
    /// 指定したテストチャネルの値を取得する
    fn get_test_value(&self, ch: TestChannelId) -> u16;
}

#[no_mangle]
pub static C2A_MONAZITE_ADC: AtomicOnceCell<&'static dyn Adc> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn ADC_initialize() -> c_int {
    let adc = C2A_MONAZITE_ADC.get();
    match adc.initialize() {
        Ok(()) => bind::ADC_ERR_CODE_ADC_OK.0,
        Err(err) => err as i32,
    }
}

#[no_mangle]
pub extern "C" fn ADC_get_value(ch: u8) -> i16 {
    let adc = C2A_MONAZITE_ADC.get();
    let Ok(ch_id) = InputChannelId::try_from(ch) else {
        return Error::ChannelError as i16;
    };
    // Safety: ADC の値は12ビットであるため、`i16` に収まる
    #[allow(clippy::cast_possible_wrap)]
    {
        adc.get_value(ch_id) as i16
    }
}

#[no_mangle]
pub extern "C" fn ADC_get_test_value(ch: u8) -> i16 {
    let adc = C2A_MONAZITE_ADC.get();
    let Ok(ch_id) = TestChannelId::try_from(ch) else {
        return Error::ChannelError as i16;
    };
    // Safety: ADC の値は12ビットであるため、`i16` に収まる
    #[allow(clippy::cast_possible_wrap)]
    {
        adc.get_test_value(ch_id) as i16
    }
}
