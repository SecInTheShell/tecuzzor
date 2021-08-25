//! -------- Types Related to Signal/IPC/Synchronization --------
use super::*;


#[cfg(feature = "type_only")]
pub type FutexOperation = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct FutexOperation(c_int);

use libc::{
    FUTEX_CLOCK_REALTIME, FUTEX_CMD_MASK, FUTEX_CMP_REQUEUE, FUTEX_CMP_REQUEUE_PI, FUTEX_FD,
    FUTEX_LOCK_PI, FUTEX_PRIVATE_FLAG, FUTEX_REQUEUE, FUTEX_TRYLOCK_PI, FUTEX_UNLOCK_PI,
    FUTEX_WAIT, FUTEX_WAIT_BITSET, FUTEX_WAIT_REQUEUE_PI, FUTEX_WAKE, FUTEX_WAKE_BITSET,
    FUTEX_WAKE_OP,
};

#[cfg(feature = "type_only")]
pub type SignalNo = c_int;

/// Check [singal(7)](https://man7.org/linux/man-pages/man7/signal.7.html)
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SignalNo(c_int);

// not in rust libc: SIGEMT, SIGCLD, SIGINFO, SIGLOST, SIGUNUSED
use libc::{
    SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGIO, SIGIOT,
    SIGKILL, SIGPIPE, SIGPOLL, SIGPROF, SIGPWR, SIGQUIT, SIGSEGV, SIGSTKFLT, SIGSTOP, SIGSYS,
    SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH,
    SIGXCPU, SIGXFSZ,
};

#[derive(Debug)]
pub struct Thread(c_int, thread::JoinHandle<()>);

use serde::Serializer;

impl Serialize for Thread {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

// impl Argument for Thread {
//     fn argumentize(&self) -> usize {
//         self.0 as usize
//     }
// }

/// Struct from C:
/// ```c
/// struct sigaction {
///     void     (*sa_handler)(int);
///     void     (*sa_sigaction)(int, siginfo_t *, void *);
///     sigset_t   sa_mask;
///     int        sa_flags;
///     void     (*sa_restorer)(void);
/// };
/// ```
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SigAction {
    // pub sa_handler:
    pub sa_mask: Sigset,
    // pub sa_flags:
    // pub sa_restore:
}


#[cfg(feature = "type_only")]
pub type SigHow = c_int;

/// See [manual](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) for explainations of the `how` argument
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SigHow(c_int);
use libc::{SIG_BLOCK, SIG_SETMASK, SIG_UNBLOCK};

/// borrowed from Rust libc `sigset_t`
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Sigset {
    #[cfg(target_pointer_width = "32")]
    __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}

use libc::sigset_t;

#[cfg(feature = "type_only")]
pub type SigsetSize = usize;


#[cfg(not(feature = "type_only"))]
#[derive(Debug, Serialize)]
/// in linux `__sigset_t.h`:
/// ```c
/// #define _SIGSET_NWORDS (1024 / (8 * sizeof (unsigned long int)))
/// typedef struct
/// {
///   unsigned long int __val[_SIGSET_NWORDS];
/// } __sigset_t;
/// ```
pub struct SigsetSize(usize);
