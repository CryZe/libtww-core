use core::fmt;
use core::ptr::null_mut;

#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_void = u8;
#[allow(non_camel_case_types)]
pub type size_t = usize;

#[no_mangle]
pub unsafe extern "C" fn __errno() -> *mut c_int {
    static mut ERRNO: c_int = 0;
    &mut ERRNO
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "eh_unwind_resume"]
pub extern "C" fn eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: fmt::Arguments, file: &str, line: u32) -> ! {
    use arrayvec::{ArrayString, ArrayVec};
    use core::fmt::Write;
    use system::OS;

    let mut text = ArrayString::<[u8; 128]>::new();
    let _ = write!(text, "Panicked at '{}', {}:{}\0", fmt, file, line);

    let mut buffer = ArrayVec::<[u8; 128]>::new();
    for &c in text.as_bytes() {
        buffer.push(c);
        if c == b'%' {
            buffer.push(b'%');
        }
    }

    OS::panic(buffer.as_ptr(), buffer.len() as i32, "HALT\0".as_ptr());
    loop {}
}

extern "C" {
    #[link_name = "cMl::memalignB(i32, u32)"]
    fn game_memalign(align: size_t, size: size_t) -> *mut c_void;
    #[link_name = "cMl::free(void*)"]
    fn game_free(ptr: *mut c_void);
    #[link_name = "strlen"]
    fn game_strlen(string: *const u8) -> size_t;
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe { game_memalign(0xFFFFFFFC, size) }
}

#[no_mangle]
pub extern "C" fn posix_memalign(
    memptr: *mut *mut c_void,
    alignment: size_t,
    size: size_t,
) -> c_int {
    unsafe {
        *memptr = game_memalign(alignment, size);
    }
    0
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe { game_free(ptr) }
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let new_data = malloc(size);

    if ptr != null_mut() {
        let mut dst = new_data as *mut u8;
        let mut src = ptr as *mut u8;

        for _ in 0..size {
            unsafe {
                *dst = *src;
                dst = dst.offset(1);
                src = src.offset(1);
            }
        }

        free(ptr);
    }

    new_data
}

#[no_mangle]
pub extern "C" fn write(_file: i32, _buffer: *const c_void, _count: size_t) -> i32 {
    unimplemented!()
}

pub fn strlen(string: *const u8) -> size_t {
    unsafe { game_strlen(string) }
}
