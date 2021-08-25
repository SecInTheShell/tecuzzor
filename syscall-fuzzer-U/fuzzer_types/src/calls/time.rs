//! Syscalls Related to Time System

use super::*;
use serde::{Deserialize, Serialize};

/// 
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Nanosleep {
    pub req: Rc::<libc::timespec>,
    pub rem: Rc::<libc::timespec>,
}
