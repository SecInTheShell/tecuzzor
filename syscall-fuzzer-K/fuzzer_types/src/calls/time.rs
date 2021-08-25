//! Syscalls Related to Time System

use super::*;
/// 
#[derive(Debug, Serialize)]
pub struct Nanosleep {
    pub req: Rc::<libc::timespec>,
    pub rem: Rc::<libc::timespec>,
}
