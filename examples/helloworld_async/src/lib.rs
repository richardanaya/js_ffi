use executor::Executor;
use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    Executor::spawn(async {
        let api = API {
            log_handle: register("console.log"),
            set_timeout_handle: register("window.setTimeout"),
        };
        api.console_log("hello");
        api.window_set_timeout(1000).await;
        api.console_log("world!");
    });
}

struct API {
    log_handle: FunctionHandle,
    set_timeout_handle: FunctionHandle,
}

impl API {
    pub fn console_log(&self, msg: &str) {
        call_1(UNDEFINED, self.log_handle, TYPE_STRING, to_js_string(msg));
    }

    pub fn window_set_timeout(&self, millis: i32) -> CallbackFuture {
        let (future, id) = CallbackFuture::new();
        call_2(
            UNDEFINED,
            self.set_timeout_handle,
            TYPE_FUNCTION,
            id,
            TYPE_NUM,
            millis as f32,
        );
        future
    }
}
