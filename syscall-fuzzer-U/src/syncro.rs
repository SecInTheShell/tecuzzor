//! Fuzz syscalls related to signal/IPC/synchronization and others

use super::*;
use dec_macro::{call, testcall, type_of};
pub use fuzzer_types::*;
use rand::rngs::StdRng;
// use syscalls::*;

pub fn syncro_test(gen: &mut StdRng) {
    for __ in 0..REPEAT {
        // rt_sigprocmask
        let (rt_sigprocmask, _res) = testcall!(Rt_sigprocmask, gen);
        println!("---- after {}: {}", type_of(&rt_sigprocmask), serde_json::to_string(&rt_sigprocmask).unwrap());

        // // rt_sigprocmask
        // // can crash the process
        // // TODO: check if Never return => cause error?
        // let (rt_sigreturn, _) = testcall!(Rt_sigreturn, gen);
        // println!("---- after lstat: {:?}", rt_sigreturn);

        // // futex
        // // TODO: check if Never return => cause error?
        // let (futex, _) = testcall!(Futex, gen);
        // println!("---- after lstat: {:?}", futex);
    }
}
