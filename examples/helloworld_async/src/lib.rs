use executor::Executor;
use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    Executor::spawn(async {
        let api = API {
            fn_log: register("console.log"),
            fn_set_timeout: register("window.setTimeout"),
        };
        api.console_log("hello");
        api.window_set_timeout(1000).await;
        api.console_log("world!");
    });
}

struct API {
    fn_log: JSFunction,
    fn_set_timeout: JSFunction,
}

impl API {
    pub fn console_log(&self, msg: &str) {
        self.fn_log.invoke_1(TYPE_STRING, to_js_string(msg));
    }

    pub fn window_set_timeout(&self, millis: i32) -> CallbackFuture {
        let (future, id) = CallbackFuture::new();
        self.fn_set_timeout
            .invoke_2(TYPE_FUNCTION, id, TYPE_NUM, millis as JSValue);
        future
    }
}
