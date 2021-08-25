#[no_std]
use super::*;
use os::raw::*;

pub fn bounded_strlen(ptr: *const u8) -> Result<usize, i32> {
    for i in 0..1000 {
        if unsafe { *ptr.offset(i) } == 0 {
            return Ok(i as _);
        }
    }

    Err(-1)
}

extern "C" {
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;

    // pub fn kmalloc(size: c_ulong, flags: c_int) -> *mut c_void;

    // pub fn kfree(objp: *const c_void);
}

// use core::alloc::{GlobalAlloc, Layout};
// struct KernelAllocator;

// unsafe impl GlobalAlloc for KernelAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         kmalloc(layout.size() as _, 0x14000c0) as _
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         kfree(ptr as _)
//     }
// }

// #[global_allocator]
// static GLOBAL: KernelAllocator = KernelAllocator;
