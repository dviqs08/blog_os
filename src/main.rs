#![no_std]
// we have to define our own panic handler
use core::panic::PanicInfo;

// we have to disable the standard library
fn main() {}

// PanicInfo contains the file and line where the panic happened
// and the optional panic message. THe function should never return,
// that's what the exclamation point means.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
