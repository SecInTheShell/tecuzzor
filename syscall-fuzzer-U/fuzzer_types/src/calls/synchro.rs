
//! Syscalls Related to Signal/IPC/Synchronization

use super::*;


// TODO: support optional type for futex
/// `long futex(uint32_t *uaddr, int futex_op, uint32_t val, const struct timespec *timeout,  /* or: uint32_t val2 */, uint32_t *uaddr2, uint32_t val3);`
/// futex - fast user-space locking
/// [Linux Manual: futex](https://man7.org/linux/man-pages/man2/futex.2.html)
#[derive(Debug, Serialize, Call, Generate)]
pub struct Futex {
    pub uaddr: Box::<u32>, //TODO
    pub futex_op: FutexOperation,
    pub val: u32,
    pub timeout: Box::<libc::timespec>, /* or: uint32_t val2 */
    pub uaddr2: Box::<u32>,
    pub val3: u32,
}

// TODO: complete `Rt_sigaction`

/// `int sigaction(int signum, const struct sigaction *restrict act, struct sigaction *restrict oldact);`
/// sigaction, rt_sigaction - examine and change a signal action
/// [Linux Manual: sigaction](https://man7.org/linux/man-pages/man2/sigaction.2.html)
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Call, Generate)]
pub struct Rt_sigaction {
    pub signum: SignalNo,
    // pub act: SigAction,
    // pub oldact: SigAction,
    // check linux source code here for more details!
    // pub size
}

// use libc::sigset_t;

/// `int rt_sigprocmask(int how, const kernel_sigset_t *set, kernel_sigset_t *oldset, size_t sigsetsize);`
/// sigprocmask, rt_sigprocmask - examine and change blocked signals
/// [Linux Manual: rt_sigprocmask](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html)
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Call, Generate)]
pub struct Rt_sigprocmask {
    pub how: SigHow,
    pub set: sigset_t,
    pub oldset: sigset_t,
    pub sigsetsize: SigsetSize,
}

/// `int sigreturn(...);`
/// sigreturn, rt_sigreturn - return from signal handler and cleanup stack frame
/// [Linux Manual: rt_sigreturn](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html)
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Call, Generate)]
pub struct Rt_sigreturn;

/// `int sched_yield(void);`
/// sched_yield - yield the processor
/// [Linux Manual: sched_yield](https://man7.org/linux/man-pages/man2/sched_yield.2.html)
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Call, Generate)]
pub struct Sched_yield;

// // TODO: 1
// /// `int tgkill(pid_t tgid, pid_t tid, int sig);`
// #[derive(Debug, Serialize, Call, Generate)]
// pub struct Tgkill {
//     pub struct tgid: ThreadGroupId,
//     pub struct tid: ThreadId,
//     pub struct sig: SignalNo,
// }

// TODO: 1
/// `int tkill(pid_t tid, int sig);`
/// tkill, tgkill - send a signal to a thread
/// tkill() is an obsolete predecessor to tgkill()
/// [Linux Manual: tkill](https://man7.org/linux/man-pages/man2/tkill.2.html)
#[derive(Debug, Serialize, Call, Generate)]
pub struct Tkill {
    pub tid: Thread,
    pub sig: SignalNo,
}

