#[no_std]
use super::*;

use os::raw::*;
// type Fd = c_int;
use os::raw::{c_ulong, c_void};
pub type Fd = c_int;
pub type BufferLength = c_ulong;

#[repr(C)]
#[derive(Serialize)]
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

#[repr(C)]
#[derive(Serialize)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Serialize)]
pub struct PollFd {
    pub fd: Fd,
    pub events: i16,
    pub revents: i16,
}

// #[repr(C)]
// #[derive(Serialize)]
// pub struct Iovec<'a> {
//     pub iov_base: &'a [u8],
//     pub iov_len: c_ulong,
// }

#[repr(C)]
#[derive(Serialize)]
pub struct Iovec {
    pub iov_base: c_ulong,
    pub iov_len: c_ulong,
}