use crate::libc;

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

// Bget allocator for *single* core systems
struct Bget {}

unsafe impl Sync for Bget {}

unsafe impl GlobalAlloc for Bget {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        libc::calloc(layout.size(), 1) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        libc::free(ptr as *mut libc::c_void)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _: Layout, new_size: usize) -> *mut u8 {
        libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
    }
}

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: Bget = Bget {};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(l) = info.location() {
        unsafe {
            libc::printf(
                "panic at %s:%d\n\0".as_ptr() as *const _,
                l.file().as_ptr(),
                l.line(),
            );
        }
    }
    unsafe { libc::abort() }
}

// #![feature(default_alloc_error_handler)] won't help.
#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    panic!("alloction failed: {:?}", layout);
}
