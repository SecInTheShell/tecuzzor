//! Fuzz syscalls related to memory management
use super::*;
use dec_macro::{call, testcall, type_of};
use fuzzer_types::*;
use rand::rngs::StdRng;

pub fn memory_test(gen: &mut StdRng) {
    for _ in 0..REPEAT {
        // mmap
        let (mmap, res) = testcall!(Mmap, gen);
        println!("---- after {}: {}", type_of(&mmap), serde_json::to_string(&mmap).unwrap());
        mmap.clean(res);

        // munmap
        let (munmap, res) = testcall!(Munmap, gen);
        println!("---- after {}: {}", type_of(&munmap), serde_json::to_string(&munmap).unwrap());
        munmap.addr.clean(res);

        // mprotect
        let (mprotect, res) = testcall!(Mprotect, gen);
        println!("---- after {}: {}", type_of(&mprotect), serde_json::to_string(&mprotect).unwrap());
        mprotect.clean(res);

        // mremap
        let (mremap, res) = testcall!(Mremap, gen);
        println!("---- after {}: {}", type_of(&mremap), serde_json::to_string(&mremap).unwrap());
        mremap.clean(res);

        // brk
        // May need to ensure the arguments are valid
        let (brk, _res) = testcall!(Brk, gen);
        println!("---- after {}: {}", type_of(&brk), serde_json::to_string(&brk).unwrap());
    }
}
