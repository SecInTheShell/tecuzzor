//! A System Call fuzzer written in Rust

use std::{thread, time};

use syscall_fuzzer::*;

use rand::rngs::StdRng;
use rand::SeedableRng;
use syscalls::SyscallNo;

use dec_macro::{call, call_libc, testcall, testcall_libc, type_of};
use fuzzer_types::calls::*;
use fuzzer_types::{Call, CallLibc, Generate};

#[macro_use]
extern crate lazy_static;

/// RNG seed for data structure generation
// static SEED: u64 = 1101;
extern crate clap;
use clap::{value_t, App, Arg};

fn init() -> (u64, u32, Option<String>, bool) {
    let matches = App::new("syscall_fuzzer")
        .version("0.1")
        .author("SecGuys")
        .about("Make semantically correct system calls")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("syscall")
                .help("The syscall No.")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .help("The seed to generate args. Default: 11037")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("The number of calls. Only works in single call mode. Default: 5")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("libc")
                .short("l")
                .help("Whether to call to the libc wrapper of this syscall")
                .required(false)
                .takes_value(false)
                .requires("syscall"),
        )
        .arg(
            Arg::with_name("sleep")
                .short("p")
                .help("sleep for several seconds before start fuzzing. Default: 3")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let seed = value_t!(matches.value_of("seed"), u64).unwrap_or(11037);
    println!("INFO: `seed` is set to {}", seed);

    let sleep = value_t!(matches.value_of("sleep"), u64).unwrap_or(3);
    println!("INFO: `sleep` is set to {}", sleep);

    let iter = value_t!(matches.value_of("number"), u32).unwrap_or(5);
    println!("INFO: `number` is set to {}", iter);

    let libc = matches.is_present("libc");

    thread::sleep(time::Duration::from_secs(sleep));

    let syscall_no = match matches.value_of("syscall") {
        Some(s) => Some(String::from(s)),
        None => None,
    };

    (seed, iter, syscall_no, libc)
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref INIT: (u64, u32, Option<String>, bool) = init();
    static ref SEED: u64 = INIT.0;
    static ref NUMBER: u32 = INIT.1;
    static ref USE_LIBC: bool = INIT.3;
}

fn main() {
    // cfg info
    #[cfg(not(feature = "type_only"))]
    println!("INFO: Fuzzer will always obey the calling conventions and consider the semantics");
    #[cfg(feature = "type_only")]
    println!(
        "INFO: Fuzzer will only follow the type definition in syscalls but ignore its semantics"
    );

    // get args

    println!("INFO: Currently configuration file has not been supported");

    match &INIT.2 {
        Some(s) => match *USE_LIBC {
            true => test_one_libc(String::from(s)),
            false => test_one(String::from(s)),
        },
        None => test_all(),
    }
}

fn test_one_libc(sysno: String) {
    let mut r = StdRng::seed_from_u64(*SEED);

    println!("INFO: Testing single syscall: {}", sysno);

    let no = match sysno.parse::<i32>() {
        Ok(no) => no,
        // TODO: convert a syscall word, e.g. read, to number
        _ => panic!("unable to find the syscall: {}", sysno),
    };

    let gen = &mut r;

    match no {
        0 => testcall_libc!(Read, gen, *NUMBER),
        1 => testcall_libc!(Write, gen, *NUMBER),
        2 => testcall_libc!(Open, gen, *NUMBER),
        3 => testcall_libc!(Close, gen, *NUMBER),
        4 => testcall_libc!(Stat, gen, *NUMBER),
        5 => testcall_libc!(Fstat, gen, *NUMBER),
        6 => testcall_libc!(Lstat, gen, *NUMBER),
        7 => testcall_libc!(Poll, gen, *NUMBER),
        8 => testcall_libc!(Lseek, gen, *NUMBER),
        9 => testcall_libc!(Mmap, gen, *NUMBER),
        10 => testcall_libc!(Mprotect, gen, *NUMBER),
        11 => testcall_libc!(Munmap, gen, *NUMBER),
        12 => testcall_libc!(Brk, gen, *NUMBER),
        17 => testcall_libc!(Pread64, gen, *NUMBER),
        18 => testcall_libc!(Pwrite64, gen, *NUMBER),
        19 => testcall_libc!(Readv, gen, *NUMBER),
        20 => testcall_libc!(Writev, gen, *NUMBER),
        21 => testcall_libc!(Access, gen, *NUMBER),
        22 => testcall_libc!(Pipe, gen, *NUMBER),
        25 => testcall_libc!(Mremap, gen, *NUMBER),
        35 => testcall_libc!(Nanosleep, gen, *NUMBER),
        39 => testcall_libc!(Getpid, gen, *NUMBER),
        41 => testcall_libc!(Socket, gen, *NUMBER),
        42 => testcall_libc!(Connect, gen, *NUMBER),
        44 => testcall_libc!(SendTo, gen, *NUMBER),
        45 => testcall_libc!(RecvFrom, gen, *NUMBER),
        54 => testcall_libc!(Setsockopt, gen, *NUMBER),
        55 => testcall_libc!(Getsockopt, gen, *NUMBER),
        102 => testcall_libc!(Getuid, gen, *NUMBER),
        107 => testcall_libc!(Geteuid, gen, *NUMBER),
        110 => testcall_libc!(Getppid, gen, *NUMBER),
        293 => testcall_libc!(Pipe2, gen, *NUMBER),
        _ => panic!("syscall number {} is not supported yet", no),
    }
}

fn test_one(sysno: String) {
    let mut r = StdRng::seed_from_u64(*SEED);

    println!("INFO: Testing single syscall: {}", sysno);

    let no = match sysno.parse::<i32>() {
        Ok(no) => no,
        // TODO: convert a syscall word, e.g. read, to number
        _ => panic!("unable to find the syscall: {}", sysno),
    };

    let gen = &mut r;

    match no {
        0 => testcall!(Read, gen, *NUMBER),
        1 => testcall!(Write, gen, *NUMBER),
        2 => testcall!(Open, gen, *NUMBER),
        3 => testcall!(Close, gen, *NUMBER),
        4 => testcall!(Stat, gen, *NUMBER),
        5 => testcall!(Fstat, gen, *NUMBER),
        6 => testcall!(Lstat, gen, *NUMBER),
        7 => testcall!(Poll, gen, *NUMBER),
        8 => testcall!(Lseek, gen, *NUMBER),
        9 => testcall!(Mmap, gen, *NUMBER),
        10 => testcall!(Mprotect, gen, *NUMBER),
        11 => testcall!(Munmap, gen, *NUMBER),
        12 => testcall!(Brk, gen, *NUMBER),
        17 => testcall!(Pread64, gen, *NUMBER),
        18 => testcall!(Pwrite64, gen, *NUMBER),
        19 => testcall!(Readv, gen, *NUMBER),
        20 => testcall!(Writev, gen, *NUMBER),
        21 => testcall!(Access, gen, *NUMBER),
        22 => testcall!(Pipe, gen, *NUMBER),
        25 => testcall!(Mremap, gen, *NUMBER),
        35 => testcall!(Nanosleep, gen, *NUMBER),
        39 => testcall!(Getpid, gen, *NUMBER),
        41 => testcall!(Socket, gen, *NUMBER),
        42 => testcall!(Connect, gen, *NUMBER),
        44 => testcall!(SendTo, gen, *NUMBER),
        45 => testcall!(RecvFrom, gen, *NUMBER),
        54 => testcall!(Setsockopt, gen, *NUMBER),
        55 => testcall!(Getsockopt, gen, *NUMBER),
        102 => testcall!(Getuid, gen, *NUMBER),
        107 => testcall!(Geteuid, gen, *NUMBER),
        110 => testcall!(Getppid, gen, *NUMBER),
        293 => testcall!(Pipe2, gen, *NUMBER),
        _ => panic!("syscall number {} is not supported yet", no),
    }
}

/// The entry point to test *nearly all* system calls in one
fn test_all() {
    println!("INFO: Testing all syscalls...");

    let mut r = StdRng::seed_from_u64(*SEED);

    // let syscalls: Vec::<Box::<dyn Call>> = vec![];

    // memory_test may not work well in `type_only` mode
    memory_test(&mut r);
    fs_test(&mut r);
    time_test(&mut r);
    syncro_test(&mut r);
    auxillary_test(&mut r);
    net_test(&mut r);
    // fuzz(&mut r);
}
