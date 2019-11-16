use executor::Executor;
use js_ffi::*;
use once_cell::sync::Lazy;
use spin::Mutex;

static JS_API: Lazy<Mutex<API>> = Lazy::new(|| {
    let a = API {
        log_handle: js!(console.log),
        set_interval_handle: js!(window.setInterval),
        set_timeout_handle: js!(window.setTimeout),
    };
    Mutex::new(a)
});

#[no_mangle]
pub fn main() -> () {
    JS_API.lock().window_set_interval(
        || {
            Executor::spawn(async {
                let api = JS_API.lock();
                api.console_log("Tic");
                api.window_set_timeout(500).await;
                api.console_log("Toc");
            });
        },
        1000,
    );
}

struct API {
    log_handle: Invoker,
    set_timeout_handle: Invoker,
    set_interval_handle: Invoker,
}

impl API {
    pub fn console_log(&self, msg: &str) {
        self.log_handle.invoke_1(TYPE_STRING, to_js_string(msg));
    }

    pub fn window_set_interval(&self, cb: impl FnMut() -> () + Send + 'static, millis: i32) {
        let id = create_callback_0(cb);
        self.set_interval_handle
            .invoke_2(TYPE_FUNCTION, id, TYPE_NUM, millis as JSValue);
    }

    pub fn window_set_timeout(&self, millis: i32) -> CallbackFuture {
        let (future, id) = CallbackFuture::new();
        self.set_timeout_handle
            .invoke_2(TYPE_FUNCTION, id, TYPE_NUM, millis as JSValue);
        future
    }
}
