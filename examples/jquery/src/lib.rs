use js_ffi::*;

#[no_mangle]
fn main() {
    let jquery_handle = register_function("$");
    let jquery_on_handle = register_function("jQuery.prototype.on");
    let alert = register_function("(msg)=>window.alert(msg)");

    let body = jquery_handle.invoke_1("body").to_js_object();
    jquery_on_handle.call_2(
        &body,
        "click",
        create_callback_1(move |_event| {
            alert.invoke_1("I was clicked!");
        }),
    );
}
