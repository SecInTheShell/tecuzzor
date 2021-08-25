//! -------- Types Related to Memory Management --------

use super::*;

// TODO: Implement debug manually to just print address
#[derive(Debug, Serialize)]
pub struct AllocatedMemory(pub Mmap);

impl Buffer for AllocatedMemory {
    fn len(&self) -> usize {
        self.0.length.argumentize()
    }
}

/// This type is the memory address which aligns to the page size
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct AlignedOffset(usize);


#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Address(pub usize);

/// Size is currently only used for memory management
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Size(pub usize);

static MAX_MALLOC_SIZE: usize = 65536;


#[repr(C)]
#[derive(Debug, Serialize)]
pub struct MapFlag(c_int);
use libc::{
    MAP_32BIT, MAP_ANON, MAP_ANONYMOUS, MAP_DENYWRITE, MAP_EXECUTABLE, MAP_FILE, MAP_FIXED,
    MAP_FIXED_NOREPLACE, MAP_GROWSDOWN, MAP_HUGETLB, MAP_HUGE_1GB, MAP_HUGE_2MB, MAP_LOCKED,
    MAP_NONBLOCK, MAP_NORESERVE, MAP_POPULATE, MAP_PRIVATE, MAP_SHARED, MAP_SHARED_VALIDATE,
    MAP_STACK, MAP_SYNC,
};


#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Protection(i32);
// Notice: no `PROT_SAO` and `PROT_SEM` in libc
use libc::{PROT_EXEC, PROT_GROWSDOWN, PROT_GROWSUP, PROT_NONE, PROT_READ, PROT_WRITE};

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct MremapFlag(c_int);
// `MREMAP_DONTUNMAP` is current not supported by rust libc
use libc::{MREMAP_FIXED, MREMAP_MAYMOVE};
