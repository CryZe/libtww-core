use core::fmt;
use core::mem::transmute;
use core::ptr::null_mut;
use Addr;

#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_void = ();
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

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let memalign =
        unsafe { transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88) };
    memalign(0xFFFFFFFC, size)
}

#[no_mangle]
pub extern "C" fn posix_memalign(
    memptr: *mut *mut c_void,
    alignment: size_t,
    size: size_t,
) -> c_int {
    let memalign =
        unsafe { transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(0x8023ea88) };
    unsafe {
        *memptr = memalign(alignment, size);
    }
    0
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    let free = unsafe { transmute::<Addr, extern "C" fn(*mut c_void)>(0x8023eac0) };
    free(ptr);
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

#[no_mangle]
pub extern "C" fn strlen(string: *const u8) -> size_t {
    let mut counter = 0;
    let mut string = string;
    while unsafe { *string } != 0 {
        string = unsafe { string.offset(1) };
        counter += 1;
    }
    counter
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
