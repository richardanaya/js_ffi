use js_ffi::*;

#[no_mangle]
fn main() {
    let btn = js!(document.querySelector).call_1(DOCUMENT, "#button");
    js!(Node.prototype.addEventListener).call_2(
        btn,
        "click",
        create_callback_0(|| {
            js!(window.alert).invoke_1("I was clicked");
        }),
    );
}
