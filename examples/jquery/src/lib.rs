use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let jquery_handle = register("$");
    // someimes functions are hidden on prototypes of things in global scope
    let jquery_on_handle = register("jQuery.prototype.on");
    // reference your own functions created in global scope
    let alert = register("(msg)=>window.alert(msg)");

    let body = jquery_handle.invoke_1(TYPE_STRING, to_js_string("body"));
    jquery_on_handle.call_2(
        body,
        TYPE_STRING,
        to_js_string("click"),
        TYPE_FUNCTION,
        create_callback_1(Box::new(move |_event| {
            alert.invoke_1(TYPE_STRING, to_js_string("I was clicked!"));
        })),
    );
}
