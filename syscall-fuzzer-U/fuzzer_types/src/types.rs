use crate::calls::*;
use fuzzer_macro::*;
use rand::{Rng, RngCore};

use super::*;
// extern crate page_size;
use std::thread;
use std::sync::mpsc::channel;
use signal_hook::iterator::Signals;
use std::os::raw::{c_char, c_int, c_long, c_short, c_ushort, c_uint, c_ulong, c_void};
use serde::Serialize;
// use serde_json::Result;

pub mod fs;
pub mod stdimpl;
pub mod time;
pub mod synchro;
pub mod mem;
pub mod net;

pub use stdimpl::*;
pub use fs::*;
pub use time::*;
pub use synchro::*;
pub use mem::*;
pub use net::*;

/// The maximum length of any buffer is set to 128
pub static MAX_BUF_LEN: usize = 16;


// -------- Meta Types --------
// Buffer Types and related BufferLength type
// Buffer needs to hold different types of data, and here the buffer can also be regarded as an array

/// This trait provide `len` method
pub trait Buffer {
    fn len(&self) -> usize;
}

/// `ArgBuffer` means to create a buffer for passing arguments to the OS.  
/// Typically, the buffer will be filled corresponding data types. 
#[derive(Debug, Serialize)]
pub struct ArgBuffer<T>(pub Vec<T>)
where
    T: Generate;

impl<T: Generate> ArgBuffer<T> {
    // to generate buffer with fixed length
    fn new(gen: &mut StdRng, len: usize) -> ArgBuffer<T> {
        let mut v = vec![];
        for _ in 0..len {
            v.push(T::generate(gen));
        }
        ArgBuffer(v)
    }
}

impl<T: Generate> Generate for ArgBuffer<T> {
    fn generate(gen: &mut StdRng) -> ArgBuffer<T> {
        let len = gen.gen_range(0..MAX_BUF_LEN);
        let mut v = vec![];
        // generate the elements
        for _ in 0..len {
            v.push(T::generate(gen));
        }
        ArgBuffer(v)
    }
}

impl<T: Generate> Argument for ArgBuffer<T> {
    fn argumentize(&self) -> usize {
        self.0.as_ptr() as *mut T as usize
    }
}

impl<T: Generate> Buffer for ArgBuffer<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(feature = "type_only")]
pub type RetBuffer<T> = ArgBuffer<T>;

/// `RetBuffer` means to create a buffer for accepting return value(s) from the OS.  
/// Typically, the buffer will be filled with zeros in normal mode.
#[cfg(not(feature = "type_only"))]
#[derive(Debug, Serialize)]
pub struct RetBuffer<T>(pub Vec<T>)
where
    T: Default;

#[cfg(not(feature = "type_only"))]
impl<T: Default> Generate for RetBuffer<T> {
    fn generate(gen: &mut StdRng) -> RetBuffer<T> {
        let len = gen.gen_range(0..MAX_BUF_LEN);
        let mut v = vec![];
        for _ in 0..len {
            v.push(T::default());
        }
        RetBuffer(v)
    }
}

#[cfg(not(feature = "type_only"))]
impl<T: Default> Argument for RetBuffer<T> {
    fn argumentize(&self) -> usize {
        self.0.as_ptr() as *mut T as usize
    }
}

#[cfg(not(feature = "type_only"))]
impl<T: Default> Buffer for RetBuffer<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// The length specifically to be used for any buffer
/// In C's syscall definition, there will always be a length/count arg following a buffer/array
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct BufferLength(pub usize);

// impl Generate for BufferLength {
//     fn generate(gen: &mut StdRng) -> BufferLength {
//         unimplemented!("Error call to generate length");
//     }
// }
