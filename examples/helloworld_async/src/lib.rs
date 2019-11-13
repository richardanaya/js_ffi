use js_ffi::*;

async fn run(){
    let console_log = js!(console.log);
    let set_timeout = js!(window.setTimeout);

    console_log.invoke_1(TYPE_STRING,to_js_string("Hello"));

    let (future, id) = CallbackFuture::new();
    set_timeout.invoke_2(TYPE_FUNCTION,id,TYPE_NUM,1000 as JSValue);
    future.await;

    console_log.invoke_1(TYPE_STRING,to_js_string("world!"));
}

#[no_mangle]
pub fn main() -> () {
    executor::spawn(run());
}