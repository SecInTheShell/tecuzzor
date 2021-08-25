#![no_std]
use super::*;

type Offset = c_long;

#[derive(Serialize)]
pub struct Open<'a> {
    pub pathname: &'a [u8],
    pub flags: c_int,
}

impl<'a> Construct for Open<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        _arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Open<'a> {
        let len = bounded_strlen(arg1 as _).unwrap();
        let v = unsafe { slice::from_raw_parts(arg1 as *const u8, len) };
        let open = Open {
            pathname: v,
            flags: arg2 as _,
        };

        let string: String<300> = serde_json_core::to_string(&open).unwrap();

        print!("[hooking] syscall:Open{}", string);

        open
    }
}

#[derive(Serialize)]
pub struct Read<'a> {
    pub fd: c_int,
    pub buf: &'a [u8],
    pub count: c_int,
}

impl<'a> Construct for Read<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Read<'a> {
        let v = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };

        let read = Read {
            fd: arg1 as _,
            buf: v,
            count: arg3 as _,
        };

        let string: String<500> = serde_json_core::to_string(&read).unwrap();

        print!("[hooking] syscall:Read{}", string);

        read
    }
}

#[derive(Serialize)]
pub struct Write<'a> {
    pub fd: c_int,
    pub buf: &'a [u8],
    pub count: c_int,
}

impl<'a> Construct for Write<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Write<'a> {
        let v = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };
        // let ptr: &[u8] = unsafe{mem::transmute(arg2)};
        // v.extend_from_slice(ptr).unwrap();
        let write = Write {
            fd: arg1 as _,
            buf: v,
            count: arg3 as _,
        };

        let string: String<500> = serde_json_core::to_string(&write).unwrap();

        print!("[hooking] syscall:Write{}", string);

        write
    }
}

#[derive(Serialize)]
pub struct Pread64<'a> {
    pub fd: c_int,
    pub buf: &'a [u8],
    pub count: c_int,
    pub offset: Offset,
}

impl<'a> Construct for Pread64<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Pread64<'a> {
        let v = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };
        // let ptr: &[u8] = unsafe{mem::transmute(arg2)};
        // v.extend_from_slice(ptr).unwrap();
        let pread64 = Pread64 {
            fd: arg1 as _,
            buf: v,
            count: arg3 as _,
            offset: arg4 as _,
        };

        let string: String<500> = serde_json_core::to_string(&pread64).unwrap();

        print!("[hooking] syscall:Pread64{}", string);

        pread64
    }
}

#[derive(Serialize)]
pub struct Pwrite64<'a> {
    pub fd: c_int,
    pub buf: &'a [u8],
    pub count: c_int,
    pub offset: Offset,
}

impl<'a> Construct for Pwrite64<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Pwrite64<'a> {
        let v = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };
        // let ptr: &[u8] = unsafe{mem::transmute(arg2)};
        // v.extend_from_slice(ptr).unwrap();
        let pwrite64 = Pwrite64 {
            fd: arg1 as _,
            buf: v,
            count: arg3 as _,
            offset: arg4 as _,
        };

        let string: String<500> = serde_json_core::to_string(&pwrite64).unwrap();

        print!("[hooking] syscall:Pwrite64{}", string);

        pwrite64
    }
}

#[derive(Serialize)]
pub struct Stat<'a> {
    pub pathname: &'a [u8],
    pub statbuf: &'static stat,
}

impl<'a> Construct for Stat<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Stat<'a> {
        let len = bounded_strlen(arg1 as _).unwrap();
        let v = unsafe { slice::from_raw_parts(arg1 as *const u8, len) };
        let stat = Stat {
            pathname: v,
            statbuf: unsafe { &*(arg2 as *const stat) },
        };

        let string: String<1000> = serde_json_core::to_string(&stat).unwrap();

        print!("[hooking] syscall:Stat{}", string);

        stat
    }
}

pub type Lstat<'a> = Stat<'a>;

#[derive(Serialize)]
pub struct Fstat {
    pub fd: c_int,
    pub statbuf: &'static stat,
}

impl Construct for Fstat {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Fstat {
        let fstat = Fstat {
            fd: arg1 as _,
            statbuf: unsafe { &*(arg2 as *const stat) },
        };

        let string: String<1000> = serde_json_core::to_string(&fstat).unwrap();

        print!("[hooking] syscall:Fstat{}", string);

        fstat
    }
}

#[derive(Serialize)]
pub struct Pipe<'a> {
    pub pipefd: &'a [Fd; 2],
}

impl<'a> Construct for Pipe<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        _arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Pipe<'a> {
        let pipe = Pipe {
            pipefd: unsafe { &*(arg1 as *const [c_int; 2]) },

        };

        let string: String<200> = serde_json_core::to_string(&pipe).unwrap();

        print!("[hooking] syscall:Pipe{}", string);

        pipe
    }
}

#[derive(Serialize)]
pub struct Pipe2<'a> {
    pub pipefd: &'a [Fd; 2],
    pub flags: c_int,
}

impl<'a> Construct for Pipe2<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Pipe2<'a> {
        let pipe2 = Pipe2 {
            pipefd: unsafe { &*(arg1 as *const [c_int; 2]) },
            flags: arg2 as _,
        };

        let string: String<200> = serde_json_core::to_string(&pipe2).unwrap();

        print!("[hooking] syscall:Pipe2{}", string);

        pipe2
    }
}

#[derive(Serialize)]
pub struct Close {
    pub fd: c_int,
}

impl Construct for Close {
    fn construct(
        arg1: os::raw::c_ulong,
        _arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Close {
        let close = Close { fd: arg1 as _ };

        let string: String<200> = serde_json_core::to_string(&close).unwrap();

        print!("[hooking] syscall:Close{}", string);

        close
    }
}

#[derive(Serialize)]
pub struct Lseek {
    pub fd: Fd,
    pub offset: Offset,
    pub whence: c_int,
}

impl Construct for Lseek {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Lseek {
        let lseek = Lseek {
            fd: arg1 as _,
            offset: arg2 as _,
            whence: arg3 as _,
        };

        let string: String<200> = serde_json_core::to_string(&lseek).unwrap();

        print!("[hooking] syscall:Lseek{}", string);

        lseek
    }
}

#[derive(Serialize)]
pub struct Access<'a> {
    pub pathname: &'a [u8],
    pub mode: c_int,
}

impl<'a> Construct for Access<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        _arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Access<'a> {
        let len = bounded_strlen(arg1 as _).unwrap();
        let v = unsafe { slice::from_raw_parts(arg1 as *const u8, len) };
        let access = Access {
            pathname: v,
            mode: arg2 as _,
        };

        let string: String<300> = serde_json_core::to_string(&access).unwrap();

        print!("[hooking] syscall:Access{}", string);

        access
    }
}

#[derive(Serialize)]
pub struct Readv<'a> {
    pub fd: Fd,
    pub iov: &'a [Iovec],
    pub iovcnt: c_int,
}

impl<'a> Construct for Readv<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Readv<'a> {

        let v = unsafe { slice::from_raw_parts(arg2 as *const Iovec, arg3 as _) };
        let readv = Readv {
            fd: arg1 as _,
            iov: v,
            iovcnt: arg3 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&readv).unwrap();

        print!("[hooking] syscall:readv{}", string);

        readv
    }
}

#[derive(Serialize)]
pub struct Writev<'a> {
    pub fd: Fd,
    pub iov: &'a [Iovec],
    pub iovcnt: c_int,
}

impl<'a> Construct for Writev<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Writev<'a> {

        let v = unsafe { slice::from_raw_parts(arg2 as *const Iovec, arg3 as _) };
        let writev = Writev {
            fd: arg1 as _,
            iov: v,
            iovcnt: arg3 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&writev).unwrap();

        print!("[hooking] syscall:writev{}", string);

        writev
    }
}

#[derive(Serialize)]
pub struct Poll<'a> {
    pub fds: &'a [PollFd],
    pub nfds: BufferLength,
    pub timeout: c_int,
}

impl<'a> Construct for Poll<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        _arg4: std::os::raw::c_ulong,
        _arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Poll<'a> {

        let v = unsafe { slice::from_raw_parts(arg1 as *const PollFd, arg2 as _) };
        let poll = Poll {
            fds: v,
            nfds: arg2 as _,
            timeout: arg3 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&poll).unwrap();
        // remember to use capital letter!!!
        print!("[hooking] syscall:Poll{}", string);

        poll
    }
}