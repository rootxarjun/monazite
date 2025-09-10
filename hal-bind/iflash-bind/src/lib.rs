#![no_std]

mod bind;

use core::{convert::Infallible, ffi::c_int};

use atomic_once_cell::AtomicOnceCell;

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Error {
    /// The arguments are not properly aligned
    NotAligned = bind::IFLASH_ERR_CODE_IFLASH_ERR_NOT_ALIGNED.0,
    /// The arguments are out of bounds
    OutOfBounds = bind::IFLASH_ERR_CODE_IFLASH_ERR_OUT_OF_BOUNDS.0,
    /// An illegal erase/program operation was attempted
    WriteProtection = bind::IFLASH_ERR_CODE_IFLASH_ERR_WRITE_PROTECTION.0,
    /// The programming sequence was incorrect
    ProgrammingSequence = bind::IFLASH_ERR_CODE_IFLASH_ERR_PROGRAMMING_SEQUENCE.0,
    /// Application software wrote several times to the same byte
    Strobe = bind::IFLASH_ERR_CODE_IFLASH_ERR_STROBE.0,
    /// Write operation was attempted before the completion of the previous
    /// write operation OR a wrap burst request overlaps two or more 256-bit
    /// flash-word addresses
    Inconsistency = bind::IFLASH_ERR_CODE_IFLASH_ERR_INCONSISTENCY.0,
    /// Error occurred during a write or erase operation
    Operation = bind::IFLASH_ERR_CODE_IFLASH_ERR_OPERATION.0,
    /// Two ECC errors detected during a read
    EccDoubleDetection = bind::IFLASH_ERR_CODE_IFLASH_ERR_ECC_DOUBLE_DETECTION.0,
    /// Read operation from a PCROP, secure-only or RDP protected area attempted
    ReadProtection = bind::IFLASH_ERR_CODE_IFLASH_ERR_READ_PROTECTION.0,
    /// Read operation from a secure address attempeted
    ReadSecure = bind::IFLASH_ERR_CODE_IFLASH_ERR_READ_SECURE.0,
    /// Other errors
    Other = bind::IFLASH_ERR_CODE_IFLASH_ERR_OTHER.0,
}

fn nb_result_to_err_code(result: nb::Result<(), Error>) -> c_int {
    match result {
        Ok(()) => bind::IFLASH_ERR_CODE_IFLASH_ERR_OK.0,
        Err(nb::Error::WouldBlock) => bind::IFLASH_ERR_CODE_IFLASH_ERR_BUSY.0,
        Err(nb::Error::Other(err)) => err as c_int,
    }
}

pub trait Iflash: Sync {
    /// # Errors
    /// 既に書き込みまたは消去が進行中の場合、[`nb::Error::WouldBlock`] を返す。
    fn start_erase(&self) -> nb::Result<(), Infallible>;

    /// # Errors
    /// 既に書き込みまたは消去が進行中の場合、[`nb::Error::WouldBlock`] を返す。
    /// `offset` または `data` が適切にアラインされていない場合、[`Error::NotAligned`] を返す。
    /// `offset` または `data` が範囲外の場合、[`Error::OutOfBounds`] を返す。
    fn start_program(&self, offset: usize, data: &[u8]) -> nb::Result<(), Error>;

    /// # Errors
    /// なんらかの操作が実行中である場合、[`nb::Error::WouldBlock`] を返す。
    /// 内蔵 Flash の操作でエラーが発生した場合、[`Error`] を返す。
    fn status(&self) -> nb::Result<(), Error>;
}

#[no_mangle]
pub static C2A_MONAZITE_IFLASH: AtomicOnceCell<&'static dyn Iflash> = AtomicOnceCell::new();

#[no_mangle]
pub extern "C" fn IFLASH_erase() -> c_int {
    let iflash = C2A_MONAZITE_IFLASH.get();
    match iflash.start_erase() {
        Ok(()) => bind::IFLASH_ERR_CODE_IFLASH_ERR_OK.0,
        Err(nb::Error::WouldBlock) => bind::IFLASH_ERR_CODE_IFLASH_ERR_BUSY.0,
        Err(nb::Error::Other(_)) => unreachable!(),
    }
}

/// # Safety
/// `data` は `len` バイトのデータを指す有効なポインタでなければならない。
#[no_mangle]
pub unsafe extern "C" fn IFLASH_program(offset: u32, data_v: *const u8, len: u32) -> c_int {
    let iflash = C2A_MONAZITE_IFLASH.get();
    let data = unsafe { core::slice::from_raw_parts(data_v, len as usize) };
    nb_result_to_err_code(iflash.start_program(offset as usize, data))
}

#[no_mangle]
pub extern "C" fn IFLASH_get_status() -> c_int {
    let iflash = C2A_MONAZITE_IFLASH.get();
    nb_result_to_err_code(iflash.status())
}
