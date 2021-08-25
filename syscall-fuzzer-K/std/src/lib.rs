// For `pub use ::core::{intrinsics,raw}` statements
#![feature(core_intrinsics)]
// #![feature(raw)]

// We *are* std
#![no_std]

#[macro_use]
pub mod macros;

// libstd-style public modules
pub mod io;
pub mod os;

// Re-export modules from libcore
pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::fmt;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::option;
pub use core::ptr;
// pub use ::core::raw;
pub use core::result;
pub use core::slice;

// Declarations to make rust-bindgen code work
mod std {
    pub use clone;
    pub use cmp;
    pub use default;
    pub use fmt;
    pub use hash;
    pub use marker;
    pub use mem;
    pub use option;
    pub use os;
    pub use slice;
}
