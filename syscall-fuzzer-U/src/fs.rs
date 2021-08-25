//! Fuzz syscalls related to File System  

use crate::REPEAT;
// use crate::call::Stat;
use dec_macro::{call, testcall, type_of};
use fuzzer_types::calls::*;
use fuzzer_types::utils::*;
// use crate::datatype::{Fd, Generate};
use rand::rngs::StdRng;
use std::{thread, time};

/// To close a generated Fd and get the result
// fn close_result_fd(rv: Result<i64, i64>) {
//     if let Ok(fd) = rv {
//         unsafe {syscall!(SYS_close, fd)};
//     }
// }

// fn close_fd(fd: datatype::Fd) {
//     unsafe {syscall!(SYS_close, fd.argumentize())};
// }

pub fn fs_test(gen: &mut StdRng) {
    for _ in 0..REPEAT {

        // open
        let (open, res) = testcall!(Open, gen);
        println!("---- after {}: {}", type_of(&open), serde_json::to_string(&open).unwrap());
        open.clean(res);

        // close
        let (close, _res) = testcall!(Close, gen);
        println!("---- after {}: {}", type_of(&close), serde_json::to_string(&close).unwrap());

        // write
        let (write, _res) = testcall!(Write, gen);
        println!("---- after {}: {}", type_of(&write), serde_json::to_string(&write).unwrap());

        // read
        let (read, _res) = testcall!(Read, gen);
        println!("---- after {}: {}", type_of(&read), serde_json::to_string(&read).unwrap());

        // fstat
        let (fstat, _res) = testcall!(Fstat, gen);
        println!("---- after {}: {}", type_of(&fstat), serde_json::to_string(&fstat).unwrap());

        // stat
        let (stat, _res) = testcall!(Stat, gen);
        println!("---- after {}: {}", type_of(&stat), serde_json::to_string(&stat).unwrap());

        // lstat
        let (lstat, _res) = testcall!(Lstat, gen);
        println!("---- after {}: {}", type_of(&lstat), serde_json::to_string(&lstat).unwrap());

        // poll
        let (poll, _res) = testcall!(Poll, gen);
        thread::sleep(time::Duration::from_millis(4000));
        println!("---- after {}: {}", type_of(&poll), serde_json::to_string(&poll).unwrap());

        // lseek
        let (lseek, _res) = testcall!(Lseek, gen);
        println!("---- after {}: {}", type_of(&lseek), serde_json::to_string(&lseek).unwrap());

        // pread64
        let (pread64, _res) = testcall!(Pread64, gen);
        println!("---- after {}: {}", type_of(&pread64), serde_json::to_string(&pread64).unwrap());

        // pwrite64
        let (pwrite64, _res) = testcall!(Pwrite64, gen);
        println!("---- after {}: {}", type_of(&pwrite64), serde_json::to_string(&pwrite64).unwrap());

        // writev
        let (writev, _res) = testcall!(Writev, gen);
        println!("---- after {}: {}", type_of(&writev), serde_json::to_string(&writev).unwrap());

        // readv
        let (readv, _res) = testcall!(Readv, gen);
        println!("---- after {}: {}", type_of(&readv), serde_json::to_string(&readv).unwrap());

        // access
        let (access, _res) = testcall!(Access, gen);
        println!("---- after {}: {}", type_of(&access), serde_json::to_string(&access).unwrap());

        // pipe
        let (pipe, _res) = testcall!(Pipe, gen);
        println!("---- after {}: {}", type_of(&pipe), serde_json::to_string(&pipe).unwrap());
    }
}
