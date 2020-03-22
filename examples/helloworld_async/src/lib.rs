#![no_std]
#![feature(alloc_error_handler)]

use js_ffi::*;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
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
#[no_mangle]
pub fn main() -> () {
    executor::spawn(async {
        let console_log = js!(console.log);
        console_log.invoke_1("Hello");
        sleep(1000).await;
        console_log.invoke_1("world!");
    });
}

fn sleep(millis: u32) -> impl core::future::Future {
    let set_timeout = js!(window.setTimeout);
    let (future, cb) = create_callback_future_0();
    set_timeout.invoke_2(cb, millis);
    future
}
