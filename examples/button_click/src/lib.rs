use js_ffi::*;

#[no_mangle]
fn main() {
    let btn = JSObject(
        js!(document.querySelector).call_1(JSGlobal::document(), JSString::from("#button")),
    );
    let cb = create_callback_0(|| {
        js!(window.alert).invoke_1(JSString::from("I was clicked"));
    });
    js!(Node.prototype.addEventListener).call_2(btn, JSString::from("click"), cb);
}
