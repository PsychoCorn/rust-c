use core::panic::PanicInfo;

use crate::*;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    eprintln!("{info}");
    unsafe { libc::abort() }
}