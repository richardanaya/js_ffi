use js_ffi::*;

#[no_mangle]
fn main() {
    let api = API::new();

    let screen = api.query_selector("#screen");
    let ctx = api.get_context(&screen, "2d");

    api.fill_style(&ctx, "red");
    api.fill_rect(&ctx, 0.0, 0.0, 50.0, 50.0);

    api.fill_style(&ctx, "green");
    api.fill_rect(&ctx, 15.0, 15.0, 50.0, 50.0);

    api.fill_style(&ctx, "blue");
    api.fill_rect(&ctx, 30.0, 30.0, 50.0, 50.0);
}

struct API {
    query_selector_handle: JSInvoker,
    get_context_handle: JSInvoker,
    fill_style_handle: JSInvoker,
    fill_rect_handle: JSInvoker,
}

impl API {
    fn new() -> API {
        API {
            query_selector_handle: js!(document.querySelector),
            get_context_handle: js!(HTMLCanvasElement.prototype.getContext),
            fill_style_handle: js!(function(color){
                this.fillStyle = color;
            }),
            fill_rect_handle: js!(CanvasRenderingContext2D.prototype.fillRect),
        }
    }

    fn query_selector(&self, s: &str) -> JSObject {
        JSObject(
            self.query_selector_handle
                .call_1(JSGlobal::document(), JSString::from(s)),
        )
    }

    fn get_context(&self, o: &JSObject, s: &str) -> JSObject {
        JSObject(self.get_context_handle.call_1(o, JSString::from(s)))
    }

    fn fill_style(&self, o: &JSObject, s: &str) {
        self.fill_style_handle.call_1(o, JSString::from(s));
    }

    fn fill_rect(&self, o: &JSObject, x: f64, y: f64, w: f64, h: f64) {
        self.fill_rect_handle.call_4(
            o,
            JSNumber::from(x),
            JSNumber::from(y),
            JSNumber::from(w),
            JSNumber::from(h),
        );
    }
}
