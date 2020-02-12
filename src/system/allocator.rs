use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, null_mut};

#[repr(C)]
struct HeapCheck {
    heap_name: *const u8,
    heap: *mut Heap,
    used: usize,
    free: usize,
    _unknown: [u8; 0x14],
}

enum Heap {}

extern "C" {
    #[link_name = "ArchiveHeapCheck"]
    static mut ArchiveHeapCheck: HeapCheck;
    #[link_name = "JKRHeap::alloc(u32, i32)"]
    fn alloc(heap: *mut Heap, size: usize, align: isize) -> *mut u8;
    #[link_name = "JKRHeap::free(void*)"]
    fn free(heap: *mut Heap, ptr: *mut u8);
}

unsafe fn calloc(size: usize, align: usize) -> *mut u8 {
    let ptr = alloc(ArchiveHeapCheck.heap, size, align as isize);
    if ptr != null_mut() {
        ptr::write_bytes(ptr, 0, size);
    }
    ptr
}

unsafe fn realloc(ptr: *mut u8, size: usize, old_size: usize, align: usize) -> *mut u8 {
    // Only actually reallocate if we are shrinking by a significant amount of
    // bytes or need to grow the buffer. This is only safe because deallocating
    // doesn't actually care about the size so we can keep around a buffer that
    // is larger than what was requested. Theoretically we don't ever need to
    // care about shrinking, but we don't have a lot of memory available in Wind
    // Waker, so at some point we should care.
    if size > old_size || old_size - size >= 32 {
        let new_data = alloc(ArchiveHeapCheck.heap, size, align as isize);

        if ptr != null_mut() {
            let dst = new_data as *mut u8;
            let src = ptr as *mut u8;

            ptr::copy_nonoverlapping(src, dst, size.min(old_size));

            free(ArchiveHeapCheck.heap, ptr);
        }

        new_data
    } else {
        ptr
    }
}

pub struct WindWakerAlloc;

#[cfg(feature = "alloc")]
unsafe impl GlobalAlloc for WindWakerAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(
            ArchiveHeapCheck.heap,
            layout.size(),
            layout.align() as isize,
        )
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        calloc(layout.size(), layout.align())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ArchiveHeapCheck.heap, ptr)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr, new_size, layout.size(), layout.align())
    }
}
