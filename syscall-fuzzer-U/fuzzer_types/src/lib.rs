//! The syscall semantics and the data types used in the syscall

pub mod types;
pub mod calls;
pub mod utils;

// pub use types::*;
pub use calls::*;
pub use utils::*;
pub use types::*;

use rand::rngs::StdRng;
use syscalls::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use rand::SeedableRng;
    use std::rc::Rc;
    #[test]
    fn it_works() {
        let mut r = StdRng::seed_from_u64(10086);
        let gen = &mut r;

        let a = <Iovec<ArgBuffer<u8>>>::generate(gen);
        println!("a: {:?}", a);
        println!("size_of IOVEC {}, size_of base: {}", size_of::<Iovec<ArgBuffer<u8>>>(), size_of::<Rc<ArgBuffer<u8>>>());
    }
}