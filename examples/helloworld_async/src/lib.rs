use js_ffi::*;

async fn run(){
    let console_log = js!(console.log);
    let set_timeout = js!(window.setTimeout);

    console_log.invoke_1(JSString::from("Hello"));

    let (future, id) = CallbackFuture::new();
    set_timeout.invoke_2(JSFunction::from(id),JSNumber::from(1000));
    future.await;

    console_log.invoke_1(JSString::from("world!"));
}

#[no_mangle]
pub fn main() -> () {
    executor::spawn(run());
}