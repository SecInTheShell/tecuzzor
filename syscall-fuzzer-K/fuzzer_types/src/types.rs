use crate::calls::*;
use fuzzer_macro::*;

use super::*;
use std::thread;
use std::sync::mpsc::channel;
use std::os::raw::{c_char, c_int, c_long, c_short, c_ushort, c_uint, c_ulong, c_void};
use serde::{Deserialize, Serialize};
// use serde_json::Result;

pub mod fs;
pub mod time;
pub mod synchro;
pub mod mem;
pub mod net;

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
pub struct ArgBuffer<T>(pub Vec<T>);

impl<T: Default> Buffer for ArgBuffer<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// `RetBuffer` means to create a buffer for accepting return value(s) from the OS.  
/// Typically, the buffer will be filled with zeros in normal mode.
#[derive(Debug, Serialize)]
pub struct RetBuffer<T>(pub Vec<T>);


impl<T: Default> Buffer for RetBuffer<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// The length specifically to be used for any buffer
/// In C's syscall definition, there will always be a length/count arg following a buffer/array
#[derive(Debug, Serialize)]
pub struct BufferLength(pub usize);
