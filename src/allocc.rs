use core::alloc::GlobalAlloc;
use libc::*;

pub struct CAlloc;

unsafe impl GlobalAlloc for CAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let allocated = malloc(layout.size());
        allocated as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
        free(ptr as *mut c_void);
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let allocated = calloc(1, layout.size());
        allocated as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, _: core::alloc::Layout, new_size: usize) -> *mut u8 {
        let reallocated = realloc(
            ptr as *mut c_void,
            new_size
        );
        reallocated as *mut u8
    }
}

#[global_allocator]
static GLOBAL: CAlloc = CAlloc;