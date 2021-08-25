
//! Auxillary syscalls

use super::*;

/// `noreturn void _exit(int status);`
/// _exit, _Exit - terminate the calling process
/// [Linux Manual: exit](https://man7.org/linux/man-pages/man2/exit.2.html)
#[derive(Debug, Serialize, Call, Generate)]
pub struct Exit {
    pub status: i32,
}

/// `void exit_group(int status);`
/// exit_group - exit all threads in a process
/// [Linux Manual: exit_group](https://man7.org/linux/man-pages/man2/exit_group.2.html)
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Call, Generate)]
pub struct Exit_group {
    pub status: i32,
}

/// `pid_t gettid(void);`
/// gettid - get thread identification
/// [Linux Manual: exit_group](https://man7.org/linux/man-pages/man2/gettid.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Gettid;

/// `pid_t vfork(void);`
/// vfork - create a child process and block parent
/// [Linux Manual: vfork](https://man7.org/linux/man-pages/man2/vfork.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Vfork;

/// `getuid() returns the real user ID of the calling process.`
/// getuid, geteuid - get user identity
/// [Linux Manual: getuid](https://man7.org/linux/man-pages/man2/getuid.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Getuid;

/// geteuid() returns the effective user ID of the calling process.
/// `uid_t geteuid(void);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Geteuid;

/// `pid_t getpid(void);`
/// getpid, getppid - get process identification
/// [Linux Manual: getpid](https://man7.org/linux/man-pages/man2/getpid.2.html)
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Getpid;

/// `pid_t getppid(void);`
#[derive(Debug, Serialize, Call, Generate, CallLibc)]
pub struct Getppid;
