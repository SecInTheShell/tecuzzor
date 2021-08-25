#[no_std]
#[panic_handler]
extern "C" fn panic_impl(info: &core::panic::PanicInfo) -> ! {
    use ::core::fmt::Write;
    use ::std::io::KernelDebugWriter;
    let mut writer = KernelDebugWriter {};

    print!("Panicked at '");

    // If this fails to write, just leave the quotes empty.
    if let Some(args) = info.message() {
        let _ = writer.write_fmt(*args);
    }

    if let Some(loc) = info.location() {
        println!("', {}:{}", loc.file(), loc.line());
    } else {
        println!("'");
    }

    // Force a null pointer read to crash.
    let _: i32 = unsafe { *(core::ptr::null()) };

    // If that doesn't work, loop forever.
    loop {}
}
