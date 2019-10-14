use executor::Executor;
use once_cell::sync::OnceCell;
use spin::Mutex;
use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    let api = get_api().lock();
    api.window_set_interval(
        Box::new(move || {
            Executor::spawn(async {
                let api = get_api().lock();
                api.console_log("Tic");
                api.window_set_timeout(500).await;
                api.console_log("Toc");
            });
        }),
        1000,
    );
}

fn get_api() -> &'static Mutex<API> {
    static INSTANCE: OnceCell<Mutex<API>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(API {
        log_handle: register("console.log"),
        set_interval_handle: register("window.setInterval"),
        set_timeout_handle: register("window.setTimeout"),
     })
    })
}

struct API {
    log_handle: FunctionHandle,
    set_timeout_handle: FunctionHandle,
    set_interval_handle: FunctionHandle,
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
            millis as f32,
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
            millis as f32,
        );
        future
    }
}
