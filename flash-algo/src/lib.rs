#![no_std]

mod bank;
mod mirrored_flasher;
mod one_side_flasher;

use flash_algorithm::ErrorCode;

const FLASH_ALGO_ERROR: ErrorCode = unsafe { ErrorCode::new_unchecked(1) };

pub use mirrored_flasher::MirroredFlasher;
pub use one_side_flasher::{Bank1, Bank2, BankSelect, OneSideFlasher};
