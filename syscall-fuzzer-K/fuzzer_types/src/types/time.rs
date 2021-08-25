//! -------- Types Related to Time System --------

use super::*;

// time_t: sec
use libc::time_t;
use libc::timespec;


#[cfg(feature = "type_only")]
pub type TimeMilliSec = c_int;

/// Time in milliseconds, int (i32)
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct TimeMilliSec(pub c_int);
