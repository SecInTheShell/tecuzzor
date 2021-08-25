//! -------- Types Related to Memory Management --------

use super::*;

const PAGE_SIZE: usize = 4096;

// TODO: Implement debug manually to just print address
#[derive(Debug, Serialize)]
pub struct AllocatedMemory(pub Mmap);

impl Generate for AllocatedMemory {
    fn generate(gen: &mut StdRng) -> AllocatedMemory {
        loop {
            let mut mmap = Mmap::generate(gen);
            let res = mmap.call();
            // this to make sure memory is allocated successfully
            match res {
                Ok(addr) => {
                    mmap.addr = Address(addr as usize);
                    return AllocatedMemory(mmap);
                }
                Err(_) => {
                    mmap.fd.clean(res);
                }
            }
        }
    }
}

// impl Drop for AllocatedMemory {
//     fn drop(&mut self) {
//         let _ = unsafe {syscall!(SYS_munmap, self.0.addr.argumentize(), self.0.length.argumentize())};
//     }
// }

impl Clean for AllocatedMemory {
    fn clean(self, res: std::result::Result<i64, i64>) {
        self.0.fd.clean(res);
        let _res = unsafe {
            syscall!(
                SYS_munmap,
                self.0.addr.argumentize(),
                self.0.length.argumentize()
            )
        };
    }
}

impl Argument for AllocatedMemory {
    fn argumentize(&self) -> usize {
        self.0.addr.argumentize()
    }
}

impl Buffer for AllocatedMemory {
    fn len(&self) -> usize {
        self.0.length.argumentize()
    }
}

/// This type is the memory address which aligns to the page size
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct AlignedOffset(usize);


impl Generate for AlignedOffset {
    fn generate(gen: &mut StdRng) -> AlignedOffset {
        let page_size = PAGE_SIZE;
        match gen.gen_range(0..2) {
            0 => AlignedOffset(0),
            _ => AlignedOffset(((gen.next_u64() as usize) / page_size) * page_size),
        }
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct Address(pub usize);

impl Generate for Address {
    fn generate(gen: &mut StdRng) -> Address {
        match gen.gen_range(0..2) {
            0 => Address(0),
            _ => Address(gen.next_u64() as _),
        }
    }
}

/// Size is currently only used for memory management
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct Size(pub usize);

static MAX_MALLOC_SIZE: usize = 65536;

impl Generate for Size {
    fn generate(gen: &mut StdRng) -> Size {
        Size(gen.gen_range(0..MAX_MALLOC_SIZE))
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct MapFlag(c_int);
use libc::{
    MAP_32BIT, MAP_ANON, MAP_ANONYMOUS, MAP_DENYWRITE, MAP_EXECUTABLE, MAP_FILE, MAP_FIXED,
    MAP_FIXED_NOREPLACE, MAP_GROWSDOWN, MAP_HUGETLB, MAP_HUGE_1GB, MAP_HUGE_2MB, MAP_LOCKED,
    MAP_NONBLOCK, MAP_NORESERVE, MAP_POPULATE, MAP_PRIVATE, MAP_SHARED, MAP_SHARED_VALIDATE,
    MAP_STACK, MAP_SYNC,
};

impl Generate for MapFlag {
    fn generate(gen: &mut StdRng) -> MapFlag {
        const MAP_FLAGS: [c_int; 21] = [
            MAP_SHARED,
            MAP_SHARED_VALIDATE,
            MAP_FIXED_NOREPLACE,
            MAP_FILE,
            MAP_PRIVATE,
            MAP_FIXED,
            MAP_HUGETLB,
            MAP_HUGE_2MB,
            MAP_HUGE_1GB,
            MAP_LOCKED,
            MAP_NORESERVE,
            MAP_32BIT,
            MAP_ANON,
            MAP_ANONYMOUS,
            MAP_DENYWRITE,
            MAP_EXECUTABLE,
            MAP_POPULATE,
            MAP_NONBLOCK,
            MAP_STACK,
            MAP_SYNC,
            MAP_GROWSDOWN,
        ];
        MapFlag(choose_one(gen, &MAP_FLAGS))
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct Protection(i32);
// Notice: no `PROT_SAO` and `PROT_SEM` in libc
use libc::{PROT_EXEC, PROT_GROWSDOWN, PROT_GROWSUP, PROT_NONE, PROT_READ, PROT_WRITE};

impl Generate for Protection {
    fn generate(gen: &mut StdRng) -> Protection {
        const PROTECTIONS: [c_int; 6] = [PROT_EXEC, PROT_GROWSDOWN, PROT_GROWSUP, PROT_NONE, PROT_READ, PROT_WRITE];
        match gen.gen_range(0..2) {
            0 => Protection(PROT_NONE),
            _ => Protection(choose_some_or(gen, &PROTECTIONS[1..])),
        }
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct MremapFlag(c_int);
// `MREMAP_DONTUNMAP` is current not supported by rust libc
use libc::{MREMAP_FIXED, MREMAP_MAYMOVE};

impl Generate for MremapFlag {
    fn generate(gen: &mut StdRng) -> MremapFlag {
        const MREMAP_FLAGS: [c_int; 2] = [MREMAP_MAYMOVE, MREMAP_FIXED];
        MremapFlag(choose_some_or(gen, &MREMAP_FLAGS))
    }
}

