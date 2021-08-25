use super::*;

#[derive(Serialize)]
pub struct Nanosleep {
    pub req: &'static timespec,
    pub rem: &'static timespec,
}

impl Construct for Nanosleep {
    fn construct(
        arg1: os::raw::c_ulong,
        arg2: os::raw::c_ulong,
        _arg3: os::raw::c_ulong,
        _arg4: os::raw::c_ulong,
        _arg5: os::raw::c_ulong,
        _arg6: os::raw::c_ulong,
    ) -> Nanosleep {
        let nanosleep = Nanosleep {
            req: unsafe { &*(arg1 as *const timespec) },
            rem: unsafe { &*(arg2 as *const timespec) },
        };
        let string: String<1000> = serde_json_core::to_string(&nanosleep).unwrap();

        print!("[hooking] syscall:Nanosleep{}", string);

        nanosleep
    }
}
