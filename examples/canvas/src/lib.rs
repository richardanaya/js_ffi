use js_ffi::*;

#[no_mangle]
fn main() {
    let api = API {
        query_selector_handle: js!(document.querySelector),
        get_context_handle: js!(HTMLCanvasElement.prototype.getContext),
        fill_style_handle: js!((s)=>{this.fillStyle = s;}),
        fill_rect_handle: js!(CanvasRenderingContext2D.prototype.fillRect),
    };

    let s = api.query_selector("#screen");
    let ctx = api.get_context(s, "2d");

    api.fill_style(ctx, "red");
    api.fill_rect(ctx, 0.0, 0.0, 50.0, 50.0);

    api.fill_style(ctx, "green");
    api.fill_rect(ctx, 15.0, 15.0, 50.0, 50.0);

    api.fill_style(ctx, "blue");
    api.fill_rect(ctx, 30.0, 30.0, 50.0, 50.0);
}

struct API {
    query_selector_handle: JSFunction,
    get_context_handle: JSFunction,
    fill_style_handle: JSFunction,
    fill_rect_handle: JSFunction,
}

impl API {
    fn query_selector(&self, s: &str) -> JSValue {
        self.query_selector_handle
            .call_1(DOCUMENT, TYPE_STRING, to_js_string(s))
    }

    fn get_context(&self, o: JSValue, s: &str) -> JSValue {
        self.get_context_handle
            .call_1(o, TYPE_STRING, to_js_string(s))
    }

    fn fill_style(&self, o: JSValue, s: &str) -> JSValue {
        self.fill_style_handle
            .call_1(o, TYPE_STRING, to_js_string(s))
    }

    fn fill_rect(&self, o: JSValue, x: f64, y: f64, w: f64, h: f64) -> JSValue {
        self.fill_rect_handle
            .call_4(o, TYPE_NUM, x, TYPE_NUM, y, TYPE_NUM, w, TYPE_NUM, h)
    }
}
