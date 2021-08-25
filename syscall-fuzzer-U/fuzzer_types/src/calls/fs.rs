//! Syscalls Related to File System

use super::*;
use crate::types::fs::stat;

/// `int open(const char *pathname, int flags, mode_t mode);`  
/// open, openat, creat - open and possibly create a file  
/// [Linux Manual: open](https://man7.org/linux/man-pages/man2/open.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Open {
    pub pathname: PathName,
    pub flags: OpenFlag,
}

/// Close the `open`ed fd
impl Clean for Open {
    fn clean(self, res: std::result::Result<i64, i64>) {
        #[cfg(not(feature = "type_only"))]
        if let Ok(fd) = res {
            let _ = Fd(fd as i32).clean(res);
        }

        #[cfg(feature = "type_only")]
        ()
    }
}

// /// Close the `open`ed fd
// impl Drop for Open {
//     fn drop(&mut self) {
//         if let Ok(fd) = res {
//             let _ = Fd(fd as i32).clean(res);
//         }
//     }
// }

/// `int close(int fd);`  
/// close - close a file descriptor  
/// [Linux Manual: close](https://man7.org/linux/man-pages/man2/close.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Close {
    pub fd: Fd,
}

/// `ssize_t read(int fd, void *buf, size_t count);`  
/// read - read from a file descriptor  
/// [Linux Manual: read](https://man7.org/linux/man-pages/man2/read.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Read {
    pub fd: Fd,
    pub buf: RetBuffer::<u8>,
    pub count: BufferLength,
}

// impl CallLibc for Read {
//     fn call_libc(&self) -> Result<i64, i64> {
//         handle_result(unsafe {
//             libc::read(
//                 self.fd.0,
//                 self.buf.argumentize() as _,
//                 self.count.argumentize(),
//             )
//         })
//     }
// }

/// `ssize_t write(int fd, const void *buf, size_t count);`  
///  write - write to a file descriptor  
/// [Linux Manual: write](https://man7.org/linux/man-pages/man2/write.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Write {
    pub fd: Fd,
    pub buf: ArgBuffer::<u8>,
    pub count: BufferLength,
}

// there are minor differences between these `stat`s

/// `int stat(const char *restrict pathname, struct stat *restrict statbuf);`  
/// stat, fstat, lstat, fstatat - get file status  
/// [Linux Manual: stat](https://man7.org/linux/man-pages/man2/stat.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Stat {
    pub pathname: PathName,
    pub statbuf: Rc::<stat>,
}

/// `int fstat(int fd, struct stat *statbuf);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Fstat {
    pub fd: Fd,
    pub statbuf: Rc::<stat>,
}

/// `int lstat(const char *restrict pathname, struct stat *restrict statbuf);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Lstat {
    pub pathname: PathName,
    pub statbuf: Rc::<stat>,
}

/// `int poll(struct pollfd *fds, nfds_t nfds, int timeout);`
/// poll, ppoll - wait for some event on a file descriptor
/// [Linux Manual: poll](https://man7.org/linux/man-pages/man2/poll.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Poll {
    pub fds: ArgBuffer::<PollFd>,
    pub nfds: BufferLength,
    pub timeout: TimeMilliSec,
}

/// `off_t lseek(int fd, off_t offset, int whence);`
/// lseek - reposition read/write file offset
/// [Linux Manual: lseek](https://man7.org/linux/man-pages/man2/lseek.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Lseek {
    pub fd: Fd,
    pub offset: Offset,
    pub whence: SeekTask,
}

/// `ssize_t pread(int fd, void *buf, size_t count, off_t offset);`
/// pread, pwrite - read from or write to a file descriptor at a given offset
/// [Linux Manual: pread](https://man7.org/linux/man-pages/man2/pread.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Pread64 {
    pub fd: Fd,
    pub buf: RetBuffer::<u8>,
    pub count: BufferLength,
    pub offset: Offset,
}

/// `ssize_t pwrite(int fd, const void *buf, size_t count, off_t offset);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Pwrite64 {
    pub fd: Fd,
    pub buf: ArgBuffer::<u8>,
    pub count: BufferLength,
    pub offset: Offset,
}

/// `ssize_t readv(int fd, const struct iovec *iov, int iovcnt);`
/// readv, writev, preadv, pwritev, preadv2, pwritev2 - read or write data into multiple buffers
/// [Linux Manual: readv](https://man7.org/linux/man-pages/man2/readv.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Readv {
    pub fd: Fd,
    pub iov: ArgBuffer::<Iovec<RetBuffer<u8>>>,
    pub iovcnt: BufferLength,
}

/// `ssize_t writev(int fd, const struct iovec *iov, int iovcnt);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Writev {
    pub fd: Fd,
    pub iov: ArgBuffer::<Iovec<RetBuffer<u8>>>,
    pub iovcnt: BufferLength,
}

// impl<'a> Generate for Writev<'a> {
//     fn generate(gen: &mut StdRng) -> Writev<'a> {
//         let buffer = ArgBuffer::<&mut Iovec>::generate(gen);
//         Writev {
//             fd: Fd::generate(gen),
//             iovcnt: BufferLength(buffer.len()),
//             iov: buffer,
//         }
//     }
// }

// impl<'a> Call for Writev<'a> {
//     fn call(&self) -> Result<i64, i64> {
//         unsafe {syscall!(SYS_writev, self.fd.argumentize(), self.iov.argumentize(), self.iovcnt.argumentize())}
//     }
// }

/// `int access(const char *pathname, int mode);`
/// access, faccessat, faccessat2 - check user's permissions for a file
/// [Linux Manual: access](https://man7.org/linux/man-pages/man2/access.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Access {
    pub pathname: PathName,
    pub mode: AccessMode,
}

/// `int pipe(int pipefd[2]);`
/// pipe, pipe2 - create pipe
/// [Linux Manual: pipe](https://man7.org/linux/man-pages/man2/pipe.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Pipe {
    pub pipefd: Rc::<[Fd; 2]>,
}
// does it need Clean?

/// `int pipe2(int pipefd[2], int flags);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Pipe2 {
    pub pipefd: Rc::<[Fd; 2]>,
    pub flags: PipeFlag,
}

// TODO: Implement Select (Generate and others)
/// `int select(int nfds, fd_set *restrict readfds, fd_set *restrict writefds, fd_set *restrict exceptfds, struct timeval *restrict timeout);`
/// select, pselect, FD_CLR, FD_ISSET, FD_SET, FD_ZERO - synchronous I/O multiplexing
/// [Linux Manual: select](https://man7.org/linux/man-pages/man2/select.2.html)
#[derive(Debug, Serialize, Call, CallLibc)]
pub struct Select {
    pub nfds: BufferLength,
    pub readfds: ArgBuffer<Fd>,
    pub writefds: ArgBuffer<Fd>,
    pub exceptfds: ArgBuffer<Fd>,
    pub timeout: ArgBuffer<TimeMilliSec>,
}

// // TODO: 1
// /// `int fcntl(int fd, int cmd, ... /* arg */ );`
// /// fcntl - manipulate file descriptor
// /// [Linux Manual: fcntl](https://man7.org/linux/man-pages/man2/fcntl.2.html)
// /// This is a *VERY* complex syscall. The optional parameter is dependent on `cmd` and can be ignored/int/*struct
// pub struct Fnctl {
//     pub fd: Fd,
//     pub cmd: FnctlCommand,
//     pub arg: Option::<i32>,
// }
