use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    let v: Vec<u8> = vec![1, 2, 3];
    let log = register("console.log");
    call_1(
        UNDEFINED,
        log,
        TYPE_UINT8_ARRAY,
        to_js_typed_array(&v).as_js_ptr(),
    );
}
