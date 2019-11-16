use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    let log = js!(console.log);
    log.invoke_1(
        TYPE_UINT8_ARRAY,
        to_js_typed_array(&vec![1u8, 2, 3]),
    );
    log.invoke_1(
        TYPE_UINT32_ARRAY,
        to_js_typed_array(&vec![4u32, 8, 12]),
    );
    log.invoke_1(
        TYPE_INT32_ARRAY,
        to_js_typed_array(&vec![-1i32, 1000, -1000]),
    );
    log.invoke_1(
        TYPE_F64_ARRAY,
        to_js_typed_array(&vec![3.14_f64, 0.0_f64, -1.0_f64]),
    );
    let a: Vec<f32> = vec![1.2, 0.0, -2.333];
    log.invoke_1(TYPE_F32_ARRAY, to_js_typed_array(&a));
}
