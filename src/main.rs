// no access to the standard library
#![no_std]
// normally Rust calls the C runtime (crt0)
// that sets up an environment for a C app,
// which creates the stack and places arguments,
// then the C runtime invokes the entry point
// of the Rust runtime.
// We don't have access to the C runtime and
// the Rust runtime, so we have to
// completely overwrite crt0.
#![no_main]

// we have to define our own panic handler
use core::panic::PanicInfo;

// we have to disable the standard library
// main is disabled.
// we are overwriting the operating system entry point.
// no_mangle so _start is literally _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// PanicInfo contains the file and line where the panic happened
// and the optional panic message. THe function should never return,
// that's what the exclamation point means.
// we also had to set the panic strategry to
// abort rather than to unwind the stack because that is
// complicated.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
