use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    // get a function handle to console log
    let log = register("console.log");
    // call it with no context and a string
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string("Hello World"));
}
