use executor::Executor;
use once_cell::sync::Lazy;
use spin::Mutex;
use js_ffi::*;

static JS_API: Lazy<Mutex<API>> = Lazy::new(|| {
    let a = API {
        log_handle: register("console.log"),
        set_interval_handle: register("window.setInterval"),
        set_timeout_handle: register("window.setTimeout"),
    };
    Mutex::new(a)
});

#[no_mangle]
pub fn main() -> () {
    JS_API.lock().window_set_interval(
        Box::new(|| {
            Executor::spawn(async {
                let api = JS_API.lock();
                api.console_log("Tic");
                api.window_set_timeout(500).await;
                api.console_log("Toc");
            });
        }),
        1000,
    );
}

struct API {
    log_handle: JSValue,
    set_timeout_handle: JSValue,
    set_interval_handle: JSValue,
}

impl API {
    pub fn console_log(&self, msg: &str) {
        call_1(UNDEFINED, self.log_handle, TYPE_STRING, to_js_string(msg));
    }

    pub fn window_set_interval(&self, cb: Box<dyn Fn() -> () + Send + 'static>, millis: i32) {
        let id = create_callback0(cb);
        call_2(
            UNDEFINED,
            self.set_interval_handle,
            TYPE_FUNCTION,
            id,
            TYPE_NUM,
            millis as JSValue,
        );
    }

    pub fn window_set_timeout(&self, millis: i32) -> CallbackFuture {
        let (future, id) = CallbackFuture::new();
        call_2(
            UNDEFINED,
            self.set_timeout_handle,
            TYPE_FUNCTION,
            id,
            TYPE_NUM,
            millis as JSValue,
        );
        future
    }
}
