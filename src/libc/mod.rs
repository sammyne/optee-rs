pub use core::ffi::c_void;

pub type size_t = usize;
pub type c_char = u8;
pub type wchar_t = u32;
pub type c_int = i32;
pub type c_uint = u32;
pub type ssize_t = isize;

// Adding link breaks compilation.
// #[link(name = "utils", kind = "static")]
// 
// This APIs comes from libutils.a built by optee_os. It seems ok without linking utils.
// Just linking the static libraries built on this library to the final TA will succeed.
// Currently, I have no idea of the underlying why.
extern "C" {
    // malloc.h
    pub fn free(p: *mut c_void);
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;

    // stdio.h
    pub fn printf(format: *const c_char, ...) -> c_int;

    // stdlib.h
    pub fn abort() -> !;
}
