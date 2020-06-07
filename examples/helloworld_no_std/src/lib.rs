#![no_std]
#![feature(alloc_error_handler)]

use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    register_function("console.log").invoke_1("Hello World");
}

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    throw_error("A panic occurred");
    loop {}
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    throw_error("Ran out of memory");
    loop {}
}
