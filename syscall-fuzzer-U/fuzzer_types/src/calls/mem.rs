//! Syscalls Related to Memory management

use super::*;
use serde::Serializer;

/// `void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);`
/// mmap, munmap - map or unmap files or devices into memory
/// [Linux Manual: mmap](https://man7.org/linux/man-pages/man2/mmap.2.html)
#[derive(Debug, Call, Generate, CallLibc)]
pub struct Mmap {
    pub addr: Address,
    pub length: Size,
    pub prot: Protection,
    pub flags: MapFlag,
    pub fd: Fd,
    pub offset: AlignedOffset,
}

impl Serialize for Mmap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.addr.0 as _)
    }
}
impl Mmap {
    // randomly allocate a part of memory *correctly*.
    // This call will try to execute `mmap` untill memory is allocated (0 as return value)
    fn ralloc(gen: &mut StdRng) -> Mmap {
        let mut succeed = false;
        let mut mmap = Mmap::generate(gen);
        while !succeed {
            mmap = Mmap::generate(gen);
            let res = mmap.call();
            if let Ok(addr) = res {
                succeed = true;
                mmap.addr = Address(addr as usize);
            }
        }
        mmap
    }

    fn dealloc(&self) {
        let _res = unsafe {syscall!(SYS_munmap, self.addr.argumentize(), self.length.argumentize())};
    }
}

impl Clean for Mmap {
    fn clean(self, res: std::result::Result<i64, i64>) {
        if let Ok(addr) = res {
            let _ = unsafe {syscall!(SYS_munmap, addr, self.length.argumentize())};
        }
    }
}

// To call `Mmap::argumentize()`, please first assign the returned address back to `Mmap.addr`
impl Argument for Mmap {
    fn argumentize(&self) -> usize {
        self.addr.argumentize()
    }
}

// /// `int munmap(void *addr, size_t length);`
// #[derive(Debug, Serialize, Call, Generate)]
// pub struct Munmap {
//     pub addr: Address,
//     pub length: Size,
// }
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Munmap {
    pub addr: AllocatedMemory,
    pub length: BufferLength,
}


// impl Generate for Munmap {
//     fn generate(gen: &mut StdRng) -> Munmap {
//         let mmap = AllocatedMemory::generate(gen);

//         Munmap {
//             length: Size(mmap.len()),
//             addr: mmap,
//         }
//     }
// }

// impl Clean for Munmap {
//     fn clean(&self, _res: Result<i64, i64>) {
//         let _ = unsafe {syscall!(SYS_munmap, self.addr.argumentize(), self.length.argumentize())};
//         // println!("munmap clean result: {:?}", res);
//     }
// }

/// `int mprotect(void *addr, size_t len, int prot);`
/// mprotect, pkey_mprotect - set protection on a region of memory
/// [Linux Manual: mprotect](https://man7.org/linux/man-pages/man2/mprotect.2.html)
#[derive(Debug, Serialize, Call, CallLibc)]
pub struct Mprotect {
    pub addr: Mmap,
    pub len: Size,
    pub prot: Protection,
}

impl Generate for Mprotect {
    fn generate(gen: &mut StdRng) -> Mprotect {
        let mmap = Mmap::ralloc(gen);
        
        Mprotect {
            len: Size(gen.gen_range(0..mmap.length.0)),
            addr: mmap,
            prot: Protection::generate(gen),
        }
    }
}

impl Clean for Mprotect {
    fn clean(self, _res: std::result::Result<i64, i64>) {
        self.addr.dealloc();
    }
}

/// `int brk(void *addr)`
/// brk, sbrk - change data segment size
/// [Linux Manual: brk](https://man7.org/linux/man-pages/man2/brk.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Brk {
    pub addr: Address,
}

/// `void *mremap(void *old_address, size_t old_size, size_t new_size, int flags, ... /* void *new_address */);`
/// mremap - remap a virtual memory address
/// [Linux Manual: mremap](https://man7.org/linux/man-pages/man2/mremap.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Mremap {
    pub old_address: Mmap,
    pub old_size: Size,
    pub new_size: Size,
    pub flags: MremapFlag,
    // TODO: find the rules for `Option` types
    // pub new_address: Option(),
}

impl Clean for Mremap {
    fn clean(self, res: std::result::Result<i64, i64>) {
        self.old_address.dealloc();
        if let Ok(addr) = res {
            let _ = unsafe {syscall!(SYS_munmap, addr, self.new_size.argumentize())};
        }
    }
}

// // TODO: 1
// /// `int madvise(void *addr, size_t length, int advice);`
// /// madvise - give advice about use of memory
// /// [Linux Manual: madvise](https://man7.org/linux/man-pages/man2/madvise.2.html)
// #[derive(Debug, Serialize, Call, Generate)]
// pub struct Madvise {
//     pub addr: Mmap,
//     pub length: Size,
//     pub advice: Advice,
// }

