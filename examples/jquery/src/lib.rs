use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let jquery_handle = js!($);
    // someimes functions are hidden on prototypes of things in global scope
    let jquery_on_handle = js!(jQuery.prototype.on);
    // reference your own functions created in global scope
    let alert = js!((msg)=>window.alert(msg));

    let body = JSObject(jquery_handle.invoke_1(JSString::from("body")));
    jquery_on_handle.call_2(
        body,
        JSString::from("click"),
        JSFunction::from(create_callback_1(Box::new(move |_event| {
            alert.invoke_1(JSString::from("I was clicked!"));
        }))),
    );
}
