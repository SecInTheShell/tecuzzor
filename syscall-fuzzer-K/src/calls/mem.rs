#![no_std]
use super::*;

#[derive(Serialize)]
pub struct Mmap {
    pub addr: c_ulong,
    pub length: c_ulong,
    pub prot: c_ulong,
    pub flags: c_ulong,
    pub fd: c_ulong,
    pub offset: c_ulong,
}

impl Construct for Mmap {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        arg4: os::raw::c_ulong,
        arg5: os::raw::c_ulong,
        arg6: os::raw::c_ulong,
    ) -> Mmap {
        let mmap = Mmap {
            addr: arg1 as _,
            length: arg2 as _,
            prot: arg3 as _,
            flags: arg4 as _,
            fd: arg5 as _,
            offset: arg6 as _,
        };

        let string: String<200> = serde_json_core::to_string(&mmap).unwrap();

        print!("[hooking] syscall:Mmap{}", string);

        mmap
    }
}

#[derive(Serialize)]
pub struct Munmap {
    pub addr: c_ulong,
    pub length: c_int,
}

impl Construct for Munmap {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Munmap {
        let munmap = Munmap {
            addr: arg1 as _,
            length: arg2 as _,
        };

        let string: String<200> = serde_json_core::to_string(&munmap).unwrap();

        print!("[hooking] syscall:Munmap{}", string);

        munmap
    }
}

#[derive(Serialize)]
pub struct Mprotect {
    pub addr: c_ulong,
    pub len: c_int,
    pub prot: c_ulong,
}

impl Construct for Mprotect {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Mprotect {
        let mprotect = Mprotect {
            addr: arg1 as _,
            len: arg2 as _,
            prot: arg3 as _,
        };

        let string: String<200> = serde_json_core::to_string(&mprotect).unwrap();

        print!("[hooking] syscall:Mprotect{}", string);

        mprotect
    }
}

#[derive(Serialize)]
pub struct Brk {
    pub addr: c_ulong,
}

impl Construct for Brk {
    fn construct(
        arg1: os::raw::c_ulong,
        _arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Brk {
        let brk = Brk { addr: arg1 as _ };

        let string: String<200> = serde_json_core::to_string(&brk).unwrap();

        print!("[hooking] syscall:Brk{}", string);

        brk
    }
}

#[derive(Serialize)]
pub struct Mremap {
    pub addr: c_ulong,
    pub old_len: c_ulong,
    pub new_len: c_ulong,
    pub flags: c_ulong,
    pub new_addr: c_ulong,
}

impl Construct for Mremap {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        arg3: os::raw::c_ulong,
        arg4: os::raw::c_ulong,
        arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Mremap {
        let mremap = Mremap {
            addr: arg1 as _,
            old_len: arg2 as _,
            new_len: arg3 as _,
            flags: arg4 as _,
            new_addr: arg5 as _,
        };

        let string: String<200> = serde_json_core::to_string(&mremap).unwrap();

        print!("[hooking] syscall:Mremap{}", string);

        mremap
    }
}
