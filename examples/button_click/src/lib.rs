use js_ffi::*;

#[no_mangle]
fn main() {
    let btn = register_function("document.querySelector").call_1(DOCUMENT, "#button").to_js_object();
    register_function("Node.prototype.addEventListener").call_2(
        btn,
        "click",
        create_callback_0(|| {
            register_function("window.alert").invoke_1("I was clicked");
        }),
    );
}
