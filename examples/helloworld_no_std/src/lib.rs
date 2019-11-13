#![no_std]
#![feature(alloc_error_handler)]

use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    js!(console.log).invoke_1(TYPE_STRING, to_js_string("Hello World"));
}

#[global_allocator]
static ALLOCATOR: malloc::Allocator = malloc::Allocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}
