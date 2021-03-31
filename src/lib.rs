#![no_std]
#![feature(alloc_error_handler, lang_items)]

extern crate alloc;

#[allow(non_camel_case_types)]
pub mod libc;

pub mod allocator;

pub use alloc::*;
