use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    let log = register_function("console.log");
    log.invoke_1(&vec![1u8, 2, 3]);
    log.invoke_1(&vec![4u32, 8, 12]);
    log.invoke_1(&vec![-1i32, 1000, -1000]);
    log.invoke_1(&vec![3.14_f64, 0.0_f64, -1.0_f64]);
    log.invoke_1(&vec![1.2, 0.0, -2.333]);
    let a = vec![1.0,2.0];
    // send array to js and send it back immediately
    let result = register_function("(data)=>data").invoke_1(&a).to_vec::<f32>();
    log.invoke_1(result.len());
    log.invoke_1(result[0]);
    log.invoke_1(result[1]);
    // make sure its same values
    assert_eq!(a,result);


    let b = vec![1u8,5,6];
    // send array to js and send it back immediately
    let result2 = register_function("(data)=>data").invoke_1(&b).to_vec::<u8>();
    // make sure its same values
    assert_eq!(b,result2);
    log.invoke_1(result2[1]);
}
