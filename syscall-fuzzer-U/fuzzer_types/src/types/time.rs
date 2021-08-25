//! -------- Types Related to Time System --------

use super::*;

// time_t: sec
use libc::time_t;
use libc::timespec;

impl Generate for time_t {
    fn generate(_gen: &mut StdRng) -> time_t {
        3
    }
}

#[cfg(feature = "type_only")]
pub type TimeMilliSec = c_int;

/// Time in milliseconds, int (i32)
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct TimeMilliSec(pub c_int);

#[cfg(not(feature = "type_only"))]
impl Generate for TimeMilliSec {
    fn generate(gen: &mut StdRng) -> TimeMilliSec {
        TimeMilliSec(gen.gen_range(0..3000))
    }
}


impl Generate for timespec {
    fn generate(gen: &mut StdRng) -> timespec {
        timespec {
            tv_sec: time_t::generate(gen),
            tv_nsec: gen.gen_range(0..999999999),
        }
    }
}