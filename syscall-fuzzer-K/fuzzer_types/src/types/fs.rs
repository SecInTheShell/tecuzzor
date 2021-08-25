use super::*;
use std::rc::Rc;

// -------- Types Related to File System --------
// open

#[cfg(feature = "type_only")]
pub type OpenFlag = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct OpenFlag(c_int);

use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use std::ffi::CStr;

pub type PathName = &'static CStr;





#[cfg(feature = "type_only")]
pub type Fd = c_int;

/// File Discriptor type
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Fd(pub i32);



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
use libc::stat;


#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PollEvent(i16);
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};


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



/// check [Linux manual: seek(2)](https://man7.org/linux/man-pages/man2/lseek.2.html) for more info
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SeekTask(i32);
use libc::{SEEK_CUR, SEEK_DATA, SEEK_END, SEEK_HOLE, SEEK_SET};


#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Offset(i64);


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
{
    pub iov_base: Rc<T>,
    pub iov_len: BufferLength,
}


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
#[derive(Debug, Serialize)]
pub struct AccessMode(i32);
use libc::{F_OK, R_OK, W_OK, X_OK};



#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PipeFd([Fd; 2]);



#[repr(C)]
#[derive(Debug, Serialize)]
pub struct PipeFlag(i32);
use libc::{O_CLOEXEC, O_DIRECT, O_NONBLOCK};
