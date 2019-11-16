use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    js!(console.log).invoke_1(JSString::from("Hello World"));
}
