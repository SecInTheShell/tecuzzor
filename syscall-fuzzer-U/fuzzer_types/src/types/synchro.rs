//! -------- Types Related to Signal/IPC/Synchronization --------
use super::*;


#[cfg(feature = "type_only")]
pub type FutexOperation = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct FutexOperation(c_int);

use libc::{
    FUTEX_CLOCK_REALTIME, FUTEX_CMD_MASK, FUTEX_CMP_REQUEUE, FUTEX_CMP_REQUEUE_PI, FUTEX_FD,
    FUTEX_LOCK_PI, FUTEX_PRIVATE_FLAG, FUTEX_REQUEUE, FUTEX_TRYLOCK_PI, FUTEX_UNLOCK_PI,
    FUTEX_WAIT, FUTEX_WAIT_BITSET, FUTEX_WAIT_REQUEUE_PI, FUTEX_WAKE, FUTEX_WAKE_BITSET,
    FUTEX_WAKE_OP,
};

#[cfg(not(feature = "type_only"))]
impl Generate for FutexOperation {
    fn generate(gen: &mut StdRng) -> FutexOperation {
        const FUTEX_OPERATIONS: [c_int; 16] = [
            FUTEX_FD,
            FUTEX_WAIT,
            FUTEX_WAKE,
            FUTEX_LOCK_PI,
            FUTEX_REQUEUE,
            FUTEX_WAKE_OP,
            FUTEX_CMD_MASK,
            FUTEX_UNLOCK_PI,
            FUTEX_TRYLOCK_PI,
            FUTEX_CMP_REQUEUE,
            FUTEX_WAIT_BITSET,
            FUTEX_WAKE_BITSET,
            FUTEX_PRIVATE_FLAG,
            FUTEX_CLOCK_REALTIME,
            FUTEX_CMP_REQUEUE_PI,
            FUTEX_WAIT_REQUEUE_PI,
        ];
        FutexOperation(choose_one(gen, &FUTEX_OPERATIONS))
    }
}

#[cfg(feature = "type_only")]
pub type SignalNo = c_int;

/// Check [singal(7)](https://man7.org/linux/man-pages/man7/signal.7.html)
#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize, Argument)]
pub struct SignalNo(c_int);

// not in rust libc: SIGEMT, SIGCLD, SIGINFO, SIGLOST, SIGUNUSED
use libc::{
    SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGIO, SIGIOT,
    SIGKILL, SIGPIPE, SIGPOLL, SIGPROF, SIGPWR, SIGQUIT, SIGSEGV, SIGSTKFLT, SIGSTOP, SIGSYS,
    SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH,
    SIGXCPU, SIGXFSZ,
};

#[cfg(not(feature = "type_only"))]
impl Generate for SignalNo {
    fn generate(gen: &mut StdRng) -> SignalNo {
        const SIGNAL_NUMBER: [c_int; 33] = [
            SIGHUP, SIGINT, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGIOT, SIGBUS, SIGFPE, SIGKILL,
            SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM, SIGTERM, SIGSTKFLT, SIGCHLD, SIGCONT,
            SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGXCPU, SIGXFSZ, SIGVTALRM, SIGPROF,
            SIGWINCH, SIGIO, SIGPOLL, SIGPWR, SIGSYS,
        ];
        SignalNo(choose_one(gen, &SIGNAL_NUMBER))
    }
}

#[derive(Debug, Argument)]
pub struct Thread(c_int, thread::JoinHandle<()>);

impl Generate for Thread {
    fn generate(_gen: &mut StdRng) -> Thread {
        let (tx, rx) = channel();
        // let mut thread_gen = gen.clone();
        let child = thread::spawn(move || {
            // let gen = &mut thread_gen;
            let allowed_signals = [
                SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGHUP, SIGINT, SIGIO, SIGPIPE,
                SIGPROF, SIGQUIT, SIGSYS, SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG,
                SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH, SIGXCPU, SIGXFSZ,
            ];
            let mut signal_iter = Signals::new(&allowed_signals).unwrap();
            tx.send(unsafe { syscall!(SYS_gettid) })
                .expect("Unable to send on CHILD channel when creating a thread");

            for signal in signal_iter.pending() {
                match signal {
                    _ if allowed_signals.contains(&signal) => break,
                    _ => continue,
                }
            }

            thread::sleep(std::time::Duration::from_millis(100));
        });

        let tid = rx
            .recv()
            .expect("Unable to receive from PARENT channel")
            .unwrap();
        Thread(tid as c_int, child)
    }
}

impl Clean for Thread {
    fn clean(self, _res: std::result::Result<i64, i64>) {
        self.1.join().unwrap();
    }
}

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
#[derive(Debug, Serialize, Argument)]
pub struct SigHow(c_int);
use libc::{SIG_BLOCK, SIG_SETMASK, SIG_UNBLOCK};

#[cfg(not(feature = "type_only"))]
impl Generate for SigHow {
    fn generate(gen: &mut StdRng) -> SigHow {
        const SIG_HOW: [c_int; 3] = [SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK];
        SigHow(choose_one(gen, &SIG_HOW))
    }
}

/// borrowed from Rust libc `sigset_t`
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Sigset {
    #[cfg(target_pointer_width = "32")]
    __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct sigset_t {
    #[cfg(target_pointer_width = "32")]
    pub __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    pub __val: [u64; 16],
}

impl Generate for sigset_t {
    fn generate(gen: &mut StdRng) -> sigset_t {
        let mut a: [u64; 16] = [0; 16];
        for i in 0..a.len() {
            a[i] = gen.next_u64();
        }
        sigset_t { __val: a }
    }
}

impl Argument for sigset_t {
    fn argumentize(&self) -> usize {
        self.__val.as_ptr() as *mut usize as usize
    }
}

#[cfg(feature = "type_only")]
pub type SigsetSize = usize;


#[cfg(not(feature = "type_only"))]
#[derive(Debug, Serialize, Argument)]
/// in linux `__sigset_t.h`:
/// ```c
/// #define _SIGSET_NWORDS (1024 / (8 * sizeof (unsigned long int)))
/// typedef struct
/// {
///   unsigned long int __val[_SIGSET_NWORDS];
/// } __sigset_t;
/// ```
pub struct SigsetSize(usize);

/// TODO: adapt architecture
#[cfg(not(feature = "type_only"))]
impl Generate for SigsetSize {
    fn generate(_gen: &mut StdRng) -> SigsetSize {
        // why it's 8 rather than 128?
        // see [sigprocmask implementation](https://code.woboq.org/userspace/glibc/sysdeps/unix/sysv/linux/sigprocmask.c.html)
        SigsetSize(8)
    }
}
