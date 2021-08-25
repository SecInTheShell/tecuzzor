#![no_std]
use super::*;

#[derive(Serialize)]
pub struct Connect<'a> {
    pub sockfd: Fd,
    pub addr: &'a [u8],
    pub addrlen: BufferLength,
}

impl<'a> Construct for Connect<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Connect<'a> {
        let v = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };

        let connect = Connect {
            sockfd: arg1 as _,
            addr: v,
            addrlen: arg3 as _,
        };

        let string: String<500> = serde_json_core::to_string(&connect).unwrap();

        print!("[hooking] syscall:Connect{}", string);

        connect
    }
}

#[derive(Serialize)]
pub struct Socket {
    pub domain: c_int,
    pub sock_type: c_int,
    pub protocol: c_int,
}

impl Construct for Socket {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Socket {
        let socket = Socket {
            domain: arg1 as _,
            sock_type: arg2 as _,
            protocol: arg3 as _,
        };

        let string: String<200> = serde_json_core::to_string(&socket).unwrap();

        print!("[hooking] syscall:Socket{}", string);

        socket
    }
}

#[derive(Serialize)]
pub struct SendTo<'a> {
    pub sockfd: Fd,
    pub buf: &'a [u8],
    pub len: BufferLength,
    pub flags: c_int,
    pub dest_addr: &'a [u8],
    pub addrlen: BufferLength,
}

impl<'a> Construct for SendTo<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        arg4: os::raw::c_ulong,
        arg5: os::raw::c_ulong,
        arg6: os::raw::c_ulong,
    ) -> SendTo<'a> {
        let v_buf = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };
        let v_dest = unsafe { slice::from_raw_parts(arg5 as *const u8, arg6 as _) };

        let sendto = SendTo {
            sockfd: arg1 as _,
            buf: v_buf,
            len: arg3 as _,
            flags: arg4 as _,
            dest_addr: v_dest,
            addrlen: arg6 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&sendto).unwrap();

        print!("[hooking] syscall:SendTo{}", string);

        sendto
    }
}

#[derive(Serialize)]
pub struct RecvFrom<'a> {
    pub sockfd: Fd,
    pub buf: &'a [u8],
    pub len: BufferLength,
    pub flags: c_int,
    pub src_addr: &'a [u8],
    pub addrlen: BufferLength,
}

impl<'a> Construct for RecvFrom<'a> {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        arg4: os::raw::c_ulong,
        arg5: os::raw::c_ulong,
        arg6: os::raw::c_ulong,
    ) -> RecvFrom<'a> {
        let v_buf = unsafe { slice::from_raw_parts(arg2 as *const u8, arg3 as _) };
        let v_dest = unsafe { slice::from_raw_parts(arg5 as *const u8, arg6 as _) };

        let recvfrom = RecvFrom {
            sockfd: arg1 as _,
            buf: v_buf,
            len: arg3 as _,
            flags: arg4 as _,
            src_addr: v_dest,
            addrlen: arg6 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&recvfrom).unwrap();

        print!("[hooking] syscall:RecvFrom{}", string);

        recvfrom
    }
}

#[derive(Serialize)]
pub struct Getsockopt<'a> {
    pub socket: Fd,
    pub level: c_int,
    pub optname: c_int,
    pub optval: &'a [u8],
    pub optlen: c_uint,
}

impl<'a> Construct for Getsockopt<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        arg4: std::os::raw::c_ulong,
        arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Getsockopt<'a> {
        let v = unsafe { slice::from_raw_parts(arg4 as *const u8, arg5 as _) };
        let len = unsafe{* (arg5 as *const c_uint)};

        let getsockopt = Getsockopt {
            socket: arg1 as _,
            level: arg2 as _,
            optname: arg3 as _,
            optval: v,
            optlen: len,
        };

        let string: String<1000> = serde_json_core::to_string(&getsockopt).unwrap();

        print!("[hooking] syscall:Getsockopt{}", string);

        getsockopt
    }
}

#[derive(Serialize)]
pub struct Setsockopt<'a> {
    pub socket: c_int,
    pub level: c_int,
    pub optname: c_int,
    pub optval: &'a [u8],
    pub optlen: c_int,
}

impl<'a> Construct for Setsockopt<'a> {
    fn construct(
        arg1: std::os::raw::c_ulong,
        arg2: std::os::raw::c_ulong,
        arg3: std::os::raw::c_ulong,
        arg4: std::os::raw::c_ulong,
        arg5: std::os::raw::c_ulong,
        _arg6: std::os::raw::c_ulong,
    ) -> Setsockopt<'a> {
        let v = unsafe { slice::from_raw_parts(arg4 as *const u8, arg5 as _) };

        let setsockopt = Setsockopt {
            socket: arg1 as _,
            level: arg2 as _,
            optname: arg3 as _,
            optval: v,
            optlen: arg5 as _,
        };

        let string: String<1000> = serde_json_core::to_string(&setsockopt).unwrap();

        print!("[hooking] syscall:Setsockopt{}", string);

        setsockopt
    }
}