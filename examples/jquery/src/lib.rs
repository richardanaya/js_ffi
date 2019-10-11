use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let jquery_handle = register("$");
    // someimes functions are hidden on prototypes of things in global scope
    let jquery_on_handle = register("jQuery.prototype.on");
    // reference your own functions created in global scope
    let alert = register("(msg)=>window.alert(msg)");

    let obj = call_1(UNDEFINED, jquery_handle, TYPE_STRING, to_js_string("body"));
    call_2(
        obj,
        jquery_on_handle,
        TYPE_STRING,
        to_js_string("click"),
        TYPE_FUNCTION,
        create_callback1(Box::new(move |_| {
            call_1(
                UNDEFINED,
                alert,
                TYPE_STRING,
                to_js_string("I was clicked!"),
            );
        })),
    );
}
