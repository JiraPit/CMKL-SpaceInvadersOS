#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use alloc::alloc::{GlobalAlloc, Layout};
pub struct Allocator;

pub static mut HEAP_START: usize = 0x0;
pub static mut HEAP_END: usize = 0x0;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocated_ptr = HEAP_START;
        HEAP_START += layout.size();
        if HEAP_START > HEAP_END {
            panic!("Out of memory");
        }
        allocated_ptr as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

pub fn init_heap(offset: usize, end: usize) {
    unsafe {
        HEAP_START = offset;
        HEAP_END = end;
    }
}
