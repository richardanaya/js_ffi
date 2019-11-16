use js_ffi::*;

async fn run() {
    let console_log = js!(console.log);
    let set_timeout = js!(window.setTimeout);

    console_log.invoke_1(JSString::from("Hello"));

    let (future, cb) = create_callback_future_0();
    set_timeout.invoke_2(cb, JSNumber::from(1000));
    future.await;

    console_log.invoke_1(JSString::from("world!"));
}

#[no_mangle]
pub fn main() -> () {
    executor::spawn(run());
}
