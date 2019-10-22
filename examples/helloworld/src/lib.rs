#![no_main]
#![no_std]
#![feature(core_intrinsics, lang_items, alloc_error_handler)]

use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    // get a function handle to console log
    let log = register("console.log");
    // call it with no context and a string
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string("Hello World"));
}

#[global_allocator]
static ALLOCATOR:malloc::Allocator = malloc::Allocator;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    loop {}
}