use js_ffi::*;

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
