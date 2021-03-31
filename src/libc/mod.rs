pub use core::ffi::c_void;

pub type size_t = usize;
pub type c_char = u8;
pub type wchar_t = u32;
pub type c_int = i32;
pub type c_uint = u32;
pub type ssize_t = isize;

extern "C" {
    // malloc.h
    pub fn free(p: *mut c_void);
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;

    // unistd.h
    //pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    // stdio.h
    pub fn printf(format: *const c_char, ...) -> c_int;

    // stdlib.h
    pub fn abort() -> !;
}
