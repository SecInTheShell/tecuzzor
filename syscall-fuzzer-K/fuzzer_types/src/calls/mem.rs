//! Syscalls Related to Memory management

use super::*;

/// `void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);`
/// mmap, munmap - map or unmap files or devices into memory
/// [Linux Manual: mmap](https://man7.org/linux/man-pages/man2/mmap.2.html)
#[derive(Debug, Serialize)]
pub struct Mmap {
    pub addr: Address,
    pub length: Size,
    pub prot: Protection,
    pub flags: MapFlag,
    pub fd: Fd,
    pub offset: AlignedOffset,
}

// /// `int munmap(void *addr, size_t length);`
// #[derive(Debug, Serialize)]
// pub struct Munmap {
//     pub addr: Address,
//     pub length: Size,
// }
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct Mprotect {
    pub addr: Mmap,
    pub len: Size,
    pub prot: Protection,
}

/// `int brk(void *addr)`
/// brk, sbrk - change data segment size
/// [Linux Manual: brk](https://man7.org/linux/man-pages/man2/brk.2.html)
#[derive(Debug, Serialize)]
pub struct Brk {
    pub addr: Address,
}

/// `void *mremap(void *old_address, size_t old_size, size_t new_size, int flags, ... /* void *new_address */);`
/// mremap - remap a virtual memory address
/// [Linux Manual: mremap](https://man7.org/linux/man-pages/man2/mremap.2.html)
#[derive(Debug, Serialize)]
pub struct Mremap {
    pub old_address: Mmap,
    pub old_size: Size,
    pub new_size: Size,
    pub flags: MremapFlag,
    // TODO: find the rules for `Option` types
    // pub new_address: Option(),
}

// // TODO: 1
// /// `int madvise(void *addr, size_t length, int advice);`
// /// madvise - give advice about use of memory
// /// [Linux Manual: madvise](https://man7.org/linux/man-pages/man2/madvise.2.html)
// #[derive(Debug, Serialize)]
// pub struct Madvise {
//     pub addr: Mmap,
//     pub length: Size,
//     pub advice: Advice,
// }

