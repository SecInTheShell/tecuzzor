//! Fuzz auxillary and misc syscalls
use super::*;
use dec_macro::{call, testcall, type_of};
pub use fuzzer_types::*;
use rand::rngs::StdRng;
use std::sync::mpsc::channel;
use std::thread::spawn;
use syscalls::*;

pub fn auxillary_test(gen: &mut StdRng) {
    // maybe this function don't need to repeat
    for _ in 0..REPEAT {
        // sched_yield
        let (sched_yield, _) = testcall!(Sched_yield, gen);
        println!("---- after {}: {}", type_of(&sched_yield), serde_json::to_string(&sched_yield).unwrap());

        // gettid
        let mut thread_gen = gen.clone();
        let (gettid, _) = spawn( move ||
            {
                let gen = &mut thread_gen;
                testcall!(Gettid, gen)
            }).join()
            .expect("Thread error occurs in `gettid syscall`");
        println!("---- after {}: {}", type_of(&gettid), serde_json::to_string(&gettid).unwrap());

        // tkill
        // can crash the process
        // let (tx, rx) = channel();
        // let mut thread_gen = gen.clone();
        // let child = spawn(move || {
        //     let gen = &mut thread_gen;
        //     tx.send(testcall!(Gettid, gen))
        //         .expect("Unable to send on CHILD channel");

        // });
        // let (tkill, res) = testcall!(Tkill, gen);
        // println!("---- after lstat: {:?}", tkill);
        // tkill.tid.clean(res);

        // let (tid, _) = rx.recv().expect("Unable to receive from PARENT channel");

        // getuid
        let (getuid, _) = testcall!(Getuid, gen);
        println!("---- after {}: {}", type_of(&getuid), serde_json::to_string(&getuid).unwrap());

        // geteuid
        let (geteuid, _) = testcall!(Geteuid, gen);
        println!("---- after {}: {}", type_of(&geteuid), serde_json::to_string(&geteuid).unwrap());

        // getpid
        let (getpid, _) = testcall!(Getpid, gen);
        println!("---- after {}: {}", type_of(&getpid), serde_json::to_string(&getpid).unwrap());

        // getppid
        let (getppid, _) = testcall!(Getppid, gen);
        println!("---- after {}: {}", type_of(&getppid), serde_json::to_string(&getppid).unwrap());
    }

    // no return functions

    // // exit
    // let (exit, _) = testcall!(Exit, gen);
    // println!("---- after lstat: {:?}", exit);

    // // vfork
    // let (vfork, _) = testcall!(Vfork, gen);
    // println!("---- after lstat: {:?}", vfork);

    // // exit_group
    // let (exit_group, _) = testcall!(Exit_group, gen);
    // println!("---- after lstat: {:?}", exit_group);
}
