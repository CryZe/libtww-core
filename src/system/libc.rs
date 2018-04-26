use core::ptr::null_mut;

extern "C" {
    #[link_name = "cMl::memalignB(i32, u32)"]
    fn game_memalign(align: isize, size: usize) -> *mut u8;
    #[link_name = "cMl::free(void*)"]
    fn game_free(ptr: *mut u8);
    #[link_name = "strlen"]
    fn game_strlen(string: *const u8) -> usize;
}

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe { game_memalign(-4, size) }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    unsafe { game_free(ptr) }
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
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
pub extern "C" fn posix_memalign(
    memptr: *mut *mut u8,
    alignment: usize,
    size: usize,
) -> i32 {
    unsafe {
        *memptr = game_memalign(alignment as isize, size);
    }
    0
}


#[no_mangle]
pub extern "C" fn write(_file: i32, _buffer: *const u8, _count: usize) -> i32 {
    unimplemented!()
}

pub fn strlen(string: *const u8) -> usize {
    unsafe { game_strlen(string) }
}
