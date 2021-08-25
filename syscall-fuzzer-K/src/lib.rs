#![no_std]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]

mod calls;
mod lang;
mod types;
mod utils;

#[macro_use]
extern crate linux_std as std;
use calls::*;
use std::*;

use serde::Serialize;

#[no_mangle]
pub extern "C" fn rust_main() {
    print!("Using print!");
}

#[no_mangle]
pub extern "C" fn syscall_logger(
    syscall_no: os::raw::c_int,
    arg1: os::raw::c_ulong,
    arg2: os::raw::c_ulong,
    arg3: os::raw::c_ulong,
    arg4: os::raw::c_ulong,
    arg5: os::raw::c_ulong,
    arg6: os::raw::c_ulong,
) {
    // print!("{}", syscall_no);
    // print!("Syscall Number: {}", syscall_no);
    // print!("Syscall Number: {:?}", syscall_no);

    match syscall_no {
        0 => {
            Read::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        1 => {
            Write::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        2 => {
            Open::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        3 => {
            Close::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        4 => {
            Stat::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        5 => {
            Fstat::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        6 => {
            Lstat::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        7 => {
            Poll::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        8 => {
            Lseek::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        9 => {
            Mmap::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        10 => {
            Mprotect::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        11 => {
            Munmap::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        12 => {
            Brk::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        17 => {
            Pread64::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        18 => {
            Pwrite64::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        19 => {
            Readv::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        20 => {
            Writev::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        21 => {
            Access::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        22 => {
            Pipe::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        25 => {
            Mremap::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        35 => {
            Nanosleep::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        41 => {
            Socket::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        42 => {
            Connect::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        44 => {
            SendTo::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        45 => {
            RecvFrom::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }        
        54 => {
            Setsockopt::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }        
        55 => {
            Getsockopt::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        }
        293 => {
            Pipe2::construct(arg1, arg2, arg3, arg4, arg5, arg6);
        } 

        _ => {
            print!("Syscall Number {:?} Not supported", syscall_no);
        }
    }
}

#[no_mangle]
pub extern "C" fn syscall_logger_input(
    syscall_no: os::raw::c_int,
    arg1: os::raw::c_ulong,
    arg2: os::raw::c_ulong,
    arg3: os::raw::c_ulong,
    arg4: os::raw::c_ulong,
    arg5: os::raw::c_ulong,
    arg6: os::raw::c_ulong,
) {
    print!("[hooking] type:input");
    syscall_logger(syscall_no, arg1, arg2, arg3, arg4, arg5, arg6);
}

#[no_mangle]
pub extern "C" fn syscall_logger_output(
    syscall_no: os::raw::c_int,
    arg1: os::raw::c_ulong,
    arg2: os::raw::c_ulong,
    arg3: os::raw::c_ulong,
    arg4: os::raw::c_ulong,
    arg5: os::raw::c_ulong,
    arg6: os::raw::c_ulong,
) {
    print!("[hooking] type:output");
    syscall_logger(syscall_no, arg1, arg2, arg3, arg4, arg5, arg6);

    // To fix the last line was not printed in kernel
    print!("");
}

pub trait Construct {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        arg4: os::raw::c_ulong,
        arg5: os::raw::c_ulong,
        arg6: os::raw::c_ulong,
    ) -> Self;
    // -> Self;
}
