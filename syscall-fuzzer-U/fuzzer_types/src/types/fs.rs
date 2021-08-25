use super::*;
use std::rc::Rc;

// -------- Types Related to File System --------
// open

#[cfg(feature = "type_only")]
pub type OpenFlag = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct OpenFlag(c_int);

use libc::{O_RDONLY, O_RDWR, O_WRONLY};

#[cfg(not(feature = "type_only"))]
impl Generate for OpenFlag {
    fn generate(gen: &mut StdRng) -> OpenFlag {
        const OPEN_FLAGS: [c_int; 3] = [O_RDWR, O_RDONLY, O_WRONLY];
        OpenFlag(choose_one(gen, &OPEN_FLAGS))
    }
}

use std::ffi::CStr;

pub type PathName = &'static CStr;

impl Generate for PathName {
    fn generate(_gen: &mut StdRng) -> PathName {
        // TODO: add more paths later
        CStr::from_bytes_with_nul(b"./test1.txt\0").unwrap()
    }
}

impl Argument for PathName {
    fn argumentize(&self) -> usize {
        self.as_ptr() as _
    }
}

#[cfg(feature = "type_only")]
pub type Fd = c_int;

/// File Discriptor type
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct Fd(pub i32);

#[cfg(not(feature = "type_only"))]
impl Generate for Fd {
    fn generate(gen: &mut StdRng) -> Fd {
        // TODO: open 0 1 2 sometimes

        let open = Open {
            pathname: PathName::generate(gen),
            flags: OpenFlag::generate(gen),
        };
        let ret = open.call();
        Fd(ret.expect("Open failed when trying to create a valid fd") as i32)
    }
}

#[cfg(not(feature = "type_only"))]
impl Clean for Fd {
    /// Close the fd
    fn clean(self, _res: std::result::Result<i64, i64>) {
        let _ = unsafe { syscall!(SYS_close, self.argumentize()) };
    }
}

#[cfg(not(feature = "type_only"))]
impl Drop for Fd {
    /// Close the fd
    fn drop(&mut self) {
        let _ = unsafe { syscall!(SYS_close, self.argumentize()) };
    }
}

/// # Originates in C
/// ```c
/// struct stat {
///     dev_t     st_dev;         /* ID of device containing file */
///     ino_t     st_ino;         /* Inode number */
///     mode_t    st_mode;        /* File type and mode */
///     nlink_t   st_nlink;       /* Number of hard links */
///     uid_t     st_uid;         /* User ID of owner */
///     gid_t     st_gid;         /* Group ID of owner */
///     dev_t     st_rdev;        /* Device ID (if special file) */
///     off_t     st_size;        /* Total size, in bytes */
///     blksize_t st_blksize;     /* Block size for filesystem I/O */
///     blkcnt_t  st_blocks;      /* Number of 512B blocks allocated */
///
///     /* Since Linux 2.6, the kernel supports nanosecond
///        precision for the following timestamp fields.
///        For the details before Linux 2.6, see NOTES. */
///
///     struct timespec st_atim;  /* Time of last access */
///     struct timespec st_mtim;  /* Time of last modification */
///     struct timespec st_ctim;  /* Time of last status change */
///
/// #define st_atime st_atim.tv_sec      /* Backward compatibility */
/// #define st_mtime st_mtim.tv_sec
/// #define st_ctime st_ctim.tv_sec
/// };
/// ```
// use libc::stat;



#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PollEvent(i16);
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};

impl Generate for PollEvent {
    fn generate(gen: &mut StdRng) -> PollEvent {
        const POLL_EVENTS: [i16; 10] = [
            POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM,
            POLLWRBAND, POLLWRNORM,
        ];
        PollEvent(choose_one(gen, &POLL_EVENTS))
    }
}

/// fd related events in *poll* originally defined in C:  
/// ```c  
/// struct pollfd {  
///     int   fd;         /* file descriptor */  
///     short events;     /* requested events */  
///     short revents;    /* returned events */  
/// };  
/// ```
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PollFd {
    pub fd: Fd,
    pub events: PollEvent,
    pub revents: PollEvent,
}

impl Generate for PollFd {
    fn generate(gen: &mut StdRng) -> PollFd {
        PollFd {
            fd: Fd::generate(gen),
            events: PollEvent::generate(gen),
            // revets will be set back to 0 by the OS to indicate timeout occurs
            revents: PollEvent(0),
        }
    }
}


/// check [Linux manual: seek(2)](https://man7.org/linux/man-pages/man2/lseek.2.html) for more info
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct SeekTask(i32);
use libc::{SEEK_CUR, SEEK_DATA, SEEK_END, SEEK_HOLE, SEEK_SET};

impl Generate for SeekTask {
    fn generate(gen: &mut StdRng) -> SeekTask {
        const SEEK_TASKS: [i32; 5] = [SEEK_SET, SEEK_CUR, SEEK_END, SEEK_DATA, SEEK_HOLE];
        SeekTask(choose_one(gen, &SEEK_TASKS))
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct Offset(i64);

impl Generate for Offset {
    /// This function may generate an offset within range `MAX_BUF_LEN` or a random u64
    fn generate(gen: &mut StdRng) -> Offset {
        match gen.gen_range(0..2) {
            0 => Offset(gen.gen_range(0..MAX_BUF_LEN) as i64),
            _ => Offset(gen.next_u64() as _),
        }
    }
}

/// Originally in C:
/// ```c
/// struct iovec {
///     void  *iov_base;    /* Starting address */
///     size_t iov_len;     /* Number of bytes to transfer */
/// };
/// ```
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Iovec<T>
where
    T: Generate + Buffer,
{
    pub iov_base: Rc<T>,
    pub iov_len: BufferLength,
}

impl<T> Generate for Iovec<T>
where
    T: Generate + Buffer,
{
    fn generate(gen: &mut StdRng) -> Iovec<T> {
        let buffer = Rc::new(T::generate(gen));
        Iovec {
            iov_len: BufferLength(buffer.len()),
            iov_base: buffer,
        }
    }
}

// #[repr(C)]
// #[derive(Debug, Serialize)]
// pub struct Iovec<'a>(pub &'a [u8]);

// impl<'a> Generate for &'a mut Iovec {
//     fn generate(gen: &mut StdRng) -> &'a mut Iovec {
//         &mut Iovec::generate(gen)
//     }
// }

/// The mode specifies the accessibility check(s) to be performed,
/// and is either the value F_OK, or a mask consisting of the bitwise
/// OR of one or more of R_OK, W_OK, and X_OK.  F_OK tests for the
/// existence of the file.  R_OK, W_OK, and X_OK test whether the
/// file exists and grants read, write, and execute permissions,
/// respectively.
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct AccessMode(i32);
use libc::{F_OK, R_OK, W_OK, X_OK};

impl Generate for AccessMode {
    fn generate(gen: &mut StdRng) -> AccessMode {
        const ACCESS_MODES: [i32; 3] = [R_OK, W_OK, X_OK];
        match gen.gen_range(0..2) {
            0 => AccessMode(F_OK),
            _ => AccessMode(choose_some_or(gen, &ACCESS_MODES[1..])),
        }
    }
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PipeFd([Fd; 2]);

#[cfg(not(feature = "type_only"))]
impl Generate for [Fd; 2] {
    fn generate(_gen: &mut StdRng) -> [Fd; 2] {
        [Fd(0), Fd(0)]
    }
}

#[cfg(feature = "type_only")]
impl Generate for [Fd; 2] {
    fn generate(gen: &mut StdRng) -> [Fd; 2] {
        [Fd::generate(gen), Fd::generate(gen)]
    }
}

// impl Generate for PipeFd {
//     fn generate(gen: &mut StdRng) -> PipeFd {
//         PipeFd([Fd(0), Fd(0)])
//     }
// }

// impl Argument for PipeFd {
//     fn argumentize(&self) -> usize {
//         self.0.as_ptr() as usize
//     }
// }

#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct PipeFlag(i32);
use libc::{O_CLOEXEC, O_DIRECT, O_NONBLOCK};

impl Generate for PipeFlag {
    fn generate(gen: &mut StdRng) -> PipeFlag {
        const PIPE_FLAGES: [i32; 3] = [O_CLOEXEC, O_DIRECT, O_NONBLOCK];
        PipeFlag(choose_some_or(gen, &PIPE_FLAGES))
    }
}


#[repr(C)]
#[derive(Debug, Serialize, Default)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    __unused: [i64; 3],
}

// TODO: may generate randomly for this type
impl Generate for stat {
    fn generate(_gen: &mut StdRng) -> stat {
        stat::default()
    }
}

impl Argument for stat {
    fn argumentize(&self) -> usize {
        self as *const _ as usize
    }
}