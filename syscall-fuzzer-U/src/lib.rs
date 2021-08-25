//! A System Call fuzzer written in Rust

// use rand::rngs::StdRng;
// use rand::Rng;

mod auxillary;
mod fs;
mod memory;
mod net;
mod randomFuzzer;
mod syncro;
mod time;

pub use auxillary::auxillary_test;
pub use fs::fs_test;
pub use memory::memory_test;
pub use net::net_test;
pub use syncro::syncro_test;
pub use time::time_test;

/// The calling times of each fuzzed syscall
pub static REPEAT: usize = 3;

// use std::os::raw::{c_int, c_bool, c_ulong, c_uint};

// pub extern "C" fn test_syscall(sysno: c_int, libc: c_bool, seed: c_ulong, repeat: c_uint) {

// }

// fn test_one_libc(sysno: String) {
//     let mut r = StdRng::seed_from_u64(*SEED);

//     println!("INFO: Testing single syscall: {}", sysno);

//     let no = match sysno.parse::<i32>() {
//         Ok(no) => no,
//         // TODO: convert a syscall word, e.g. read, to number
//         _ => panic!("unable to find the syscall: {}", sysno),
//     };

//     let gen = &mut r;

//     match no {
//         0 => testcall_libc!(Read, gen, *NUMBER),
//         1 => testcall_libc!(Write, gen, *NUMBER),
//         2 => testcall_libc!(Open, gen, *NUMBER),
//         3 => testcall_libc!(Close, gen, *NUMBER),
//         4 => testcall_libc!(Stat, gen, *NUMBER),
//         5 => testcall_libc!(Fstat, gen, *NUMBER),
//         6 => testcall_libc!(Lstat, gen, *NUMBER),
//         7 => testcall_libc!(Poll, gen, *NUMBER),
//         8 => testcall_libc!(Lseek, gen, *NUMBER),
//         9 => testcall_libc!(Mmap, gen, *NUMBER),
//         10 => testcall_libc!(Mprotect, gen, *NUMBER),
//         11 => testcall_libc!(Munmap, gen, *NUMBER),
//         12 => testcall_libc!(Brk, gen, *NUMBER),
//         17 => testcall_libc!(Pread64, gen, *NUMBER),
//         18 => testcall_libc!(Pwrite64, gen, *NUMBER),
//         19 => testcall_libc!(Readv, gen, *NUMBER),
//         20 => testcall_libc!(Writev, gen, *NUMBER),
//         21 => testcall_libc!(Access, gen, *NUMBER),
//         22 => testcall_libc!(Pipe, gen, *NUMBER),
//         25 => testcall_libc!(Mremap, gen, *NUMBER),
//         35 => testcall_libc!(Nanosleep, gen, *NUMBER),
//         39 => testcall_libc!(Getpid, gen, *NUMBER),
//         41 => testcall_libc!(Socket, gen, *NUMBER),
//         42 => testcall_libc!(Connect, gen, *NUMBER),
//         44 => testcall_libc!(SendTo, gen, *NUMBER),
//         45 => testcall_libc!(RecvFrom, gen, *NUMBER),
//         54 => testcall_libc!(Setsockopt, gen, *NUMBER),
//         55 => testcall_libc!(Getsockopt, gen, *NUMBER),
//         102 => testcall_libc!(Getuid, gen, *NUMBER),
//         107 => testcall_libc!(Geteuid, gen, *NUMBER),
//         110 => testcall_libc!(Getppid, gen, *NUMBER),
//         293 => testcall_libc!(Pipe2, gen, *NUMBER),
//         _ => panic!("syscall number {} is not supported yet", no),
//     }
// }

// fn test_one(sysno: String) {
//     let mut r = StdRng::seed_from_u64(*SEED);

//     println!("INFO: Testing single syscall: {}", sysno);

//     let no = match sysno.parse::<i32>() {
//         Ok(no) => no,
//         // TODO: convert a syscall word, e.g. read, to number
//         _ => panic!("unable to find the syscall: {}", sysno),
//     };

//     let gen = &mut r;

//     match no {
//         0 => testcall!(Read, gen, *NUMBER),
//         1 => testcall!(Write, gen, *NUMBER),
//         2 => testcall!(Open, gen, *NUMBER),
//         3 => testcall!(Close, gen, *NUMBER),
//         4 => testcall!(Stat, gen, *NUMBER),
//         5 => testcall!(Fstat, gen, *NUMBER),
//         6 => testcall!(Lstat, gen, *NUMBER),
//         7 => testcall!(Poll, gen, *NUMBER),
//         8 => testcall!(Lseek, gen, *NUMBER),
//         9 => testcall!(Mmap, gen, *NUMBER),
//         10 => testcall!(Mprotect, gen, *NUMBER),
//         11 => testcall!(Munmap, gen, *NUMBER),
//         12 => testcall!(Brk, gen, *NUMBER),
//         17 => testcall!(Pread64, gen, *NUMBER),
//         18 => testcall!(Pwrite64, gen, *NUMBER),
//         19 => testcall!(Readv, gen, *NUMBER),
//         20 => testcall!(Writev, gen, *NUMBER),
//         21 => testcall!(Access, gen, *NUMBER),
//         22 => testcall!(Pipe, gen, *NUMBER),
//         25 => testcall!(Mremap, gen, *NUMBER),
//         35 => testcall!(Nanosleep, gen, *NUMBER),
//         39 => testcall!(Getpid, gen, *NUMBER),
//         41 => testcall!(Socket, gen, *NUMBER),
//         42 => testcall!(Connect, gen, *NUMBER),
//         44 => testcall!(SendTo, gen, *NUMBER),
//         45 => testcall!(RecvFrom, gen, *NUMBER),
//         54 => testcall!(Setsockopt, gen, *NUMBER),
//         55 => testcall!(Getsockopt, gen, *NUMBER),
//         102 => testcall!(Getuid, gen, *NUMBER),
//         107 => testcall!(Geteuid, gen, *NUMBER),
//         110 => testcall!(Getppid, gen, *NUMBER),
//         293 => testcall!(Pipe2, gen, *NUMBER),
//         _ => panic!("syscall number {} is not supported yet", no),
//     }
// }


// pub static REPEAT: usize = 3;

// #[cfg(test)]
// mod tests {
//     use rand::rngs::StdRng;
//     use rand::SeedableRng;

//     #[test]
//     fn testSyscall() {
//         const SEED: u64 = 11037;
//         let mut r = StdRng::seed_from_u64(SEED);
//         let tc = Pread64::generate(&mut r);

//         let rv = tc.call();
//         println!("rv from test call: {:?}", rv);
//     }
// }
