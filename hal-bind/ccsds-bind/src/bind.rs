#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]

#[allow(clippy::wildcard_imports)]
use core::*;
include!(concat!(env!("OUT_DIR"), "/ccsds.rs"));
