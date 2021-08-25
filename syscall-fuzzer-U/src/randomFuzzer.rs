use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use syscalls::*;

// fn gen_param(gen: &mut StdRng) -> u64 {

// }

fn gen_mem(gen: &mut StdRng) -> u64 {
    let mut mem: Box<u64> = Box::new(gen.next_u64());
    return &mut mem as *mut _ as u64;
}

pub fn fuzz(gen: &mut StdRng) {
    for i in 0..10 {
        println!("{}th random syscall:", i);
        let arg_count = gen.gen_range(0..6);
        let syscall_no = SyscallNo::new(gen.gen_range(0..400) as usize);
        let syscall_no = match syscall_no {
            Some(no) => no,
            _ => continue,
        };
        let mem = gen_mem(gen);
        unsafe {
            syscall1(syscall_no, mem);
        }
    }
}
