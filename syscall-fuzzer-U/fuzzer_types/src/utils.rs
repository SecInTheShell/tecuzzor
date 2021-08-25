use rand::rngs::StdRng;
use rand::Rng;

/// Structs implement this trait can generate a instance(variable) automatically
pub trait Generate {
    /// Take a random number generator and then generate a instance
    /// - For syscalls, `generate` can be implemented automatically by deriving `Generate`, and parameters of different datatypes are generated iteratively
    /// - For datatypes used in syscalls, the `generate` method must be implemented manually
    fn generate(gen: &mut StdRng) -> Self;
}

/// Must be implemented manually for all datatypes that can appear in syscalls
pub trait Argument {
    /// To convert datatypes to appropriate parameters in syscalls
    fn argumentize(&self) -> usize;
}

/// Structs implemented this trait can make a syscall
/// The syscall name is also the name of the struct
pub trait Call {
    fn call(&self) -> Result<i64, i64>;
}

/// Structs implemented this trait can make a syscall using libc wrapper
pub trait CallLibc {
    fn call_libc(&self) -> Result<i64, i64>;
}

pub fn handle_result(ret: isize) -> Result<i64, i64> {
    if ret >= 0 {
        Ok(ret as _)
    } else {
        Err(-ret as _)
    }
}

/// Structs in `call.rs` implementing this trait provide a method to eliminate potential side effects of a syscall
/// Structs in `datatype.rs` implementing this trait also do some cleaning jobs
pub trait Clean {
    // maybe clean should take the ownership?
    fn clean(self, res: Result<i64, i64>);
}

pub fn choose_one<T>(gen: &mut StdRng, a: &[T]) -> T
where
    T: Copy,
{
    a[gen.gen_range(0..a.len())]
}

pub fn choose_some_or<T>(gen: &mut StdRng, a: &[T]) -> T
where
    T: Copy + std::ops::BitOr<T> + std::ops::BitOr<Output = T>,
{
    let mut res = a[gen.gen_range(0..a.len())];
    for _ in 0..(a.len() - 1) {
        res = res | a[gen.gen_range(0..a.len())];
    }
    res
}
