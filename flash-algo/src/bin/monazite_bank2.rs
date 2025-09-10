#![no_std]
#![no_main]

use flash_algorithm::{algorithm, count, FlashAlgorithm};
use monazite_flash_algo::{Bank2, OneSideFlasher};

algorithm!(OneSideFlasher<Bank2>, {
    flash_address: 0x0800_0000,
    flash_size: 0x0010_0000,
    page_size: 1024,
    empty_value: 0xFF,
    sectors: [{
        size: 0x20000,
        address: 0,
    }]
});
