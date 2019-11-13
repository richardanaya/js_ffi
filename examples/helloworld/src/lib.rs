use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    js!(console.log).invoke_1(TYPE_STRING, to_js_string("Hello World"));
}