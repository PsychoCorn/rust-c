#![no_std]

pub extern crate alloc;

pub mod panic;
pub mod allocc;
#[macro_use]
pub mod io;
pub mod fs;

use core::{ffi::CStr, ptr::slice_from_raw_parts};

use alloc::vec::Vec;
pub use allocc::*;
pub use panic::*;


/// # Safety
/// argc and argv must be main arguments
pub unsafe fn args_to_vec(argc: libc::c_int, argv: *const *const libc::c_char) -> Vec<&'static CStr> {
    // SAFETY:
    // if argc and argv are real cmd arguments
    // this is safety
    unsafe {
        &*slice_from_raw_parts(
            argv, argc as usize
        )
    }.into_iter().map(|x| {
        unsafe {
            CStr::from_ptr(*x)
        }
    }).collect()
}