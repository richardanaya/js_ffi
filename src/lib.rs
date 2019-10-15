#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use cstring::{cstr, cstr_to_string};
pub use callback::*;
pub use wasm_common::*;

extern "C" {
    fn jsffirelease(obj: JSValue);
    fn jsffiregister(code: CStrPtr) -> JSValue;
    fn jsfficall0(obj: JSValue, function: JSValue) -> JSValue;
    fn jsfficall1(obj: JSValue, function: JSValue, a1_type: JSType, a1: JSValue) -> JSValue;
    fn jsfficall2(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
    ) -> JSValue;
    fn jsfficall3(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
    ) -> JSValue;
    fn jsfficall4(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
    ) -> JSValue;
    fn jsfficall5(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
    ) -> JSValue;
    fn jsfficall6(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
        a6_type: JSType,
        a6: JSValue,
    ) -> JSValue;
    fn jsfficall7(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
        a6_type: JSType,
        a6: JSValue,
        a7_type: JSType,
        a7: JSValue,
    ) -> JSValue;
    fn jsfficall8(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
        a6_type: JSType,
        a6: JSValue,
        a7_type: JSType,
        a7: JSValue,
        a8_type: JSType,
        a8: JSValue,
    ) -> JSValue;
    fn jsfficall9(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
        a6_type: JSType,
        a6: JSValue,
        a7_type: JSType,
        a7: JSValue,
        a8_type: JSType,
        a8: JSValue,
        a9_type: JSType,
        a9: JSValue,
    ) -> JSValue;
    fn jsfficall10(
        obj: JSValue,
        function: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
        a4_type: JSType,
        a4: JSValue,
        a5_type: JSType,
        a5: JSValue,
        a6_type: JSType,
        a6: JSValue,
        a7_type: JSType,
        a7: JSValue,
        a8_type: JSType,
        a8: JSValue,
        a9_type: JSType,
        a9: JSValue,
        a10_type: JSType,
        a10: JSValue,
    ) -> JSValue;
}

pub const NULL: JSValue = 1.0;
pub const CONSOLE: JSValue = 2.0;
pub const WINDOW: JSValue = 3.0;
pub const DOCUMENT: JSValue = 4.0;
pub type CStrPtr = i32;
pub type JSType = i32;

pub const TYPE_NOTHING: JSType = 0;
pub const TYPE_NUM: JSType = 1;
pub const TYPE_STRING: JSType = 2;
pub const TYPE_BOOL: JSType = 3;
pub const TYPE_FUNCTION: JSType = 4;
pub const TYPE_OBJECT: JSType = 5;

pub fn release(obj: JSValue) {
    unsafe { jsffirelease(obj) }
}

pub fn register(code: &str) -> JSValue {
    unsafe { jsffiregister(cstr(code)) }
}

pub fn call_0(obj: JSValue, function: JSValue) -> JSValue {
    unsafe { jsfficall0(obj, function) }
}

pub fn call_1(obj: JSValue, function: JSValue, a1_type: JSType, a1: JSValue) -> JSValue {
    unsafe { jsfficall1(obj, function, a1_type, a1) }
}

pub fn call_2(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
) -> JSValue {
    unsafe { jsfficall2(obj, function, a1_type, a1, a2_type, a2) }
}

pub fn call_3(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
) -> JSValue {
    unsafe { jsfficall3(obj, function, a1_type, a1, a2_type, a2, a3_type, a3) }
}

pub fn call_4(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
) -> JSValue {
    unsafe {
        jsfficall4(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4,
        )
    }
}

pub fn call_5(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
) -> JSValue {
    unsafe {
        jsfficall5(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
        )
    }
}

pub fn call_6(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
    a6_type: JSType,
    a6: JSValue,
) -> JSValue {
    unsafe {
        jsfficall6(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6,
        )
    }
}

pub fn call_7(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
    a6_type: JSType,
    a6: JSValue,
    a7_type: JSType,
    a7: JSValue,
) -> JSValue {
    unsafe {
        jsfficall7(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6, a7_type, a7,
        )
    }
}

pub fn call_8(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
    a6_type: JSType,
    a6: JSValue,
    a7_type: JSType,
    a7: JSValue,
    a8_type: JSType,
    a8: JSValue,
) -> JSValue {
    unsafe {
        jsfficall8(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6, a7_type, a7, a8_type, a8,
        )
    }
}

pub fn call_9(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
    a6_type: JSType,
    a6: JSValue,
    a7_type: JSType,
    a7: JSValue,
    a8_type: JSType,
    a8: JSValue,
    a9_type: JSType,
    a9: JSValue,
) -> JSValue {
    unsafe {
        jsfficall9(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9,
        )
    }
}

pub fn call_10(
    obj: JSValue,
    function: JSValue,
    a1_type: JSType,
    a1: JSValue,
    a2_type: JSType,
    a2: JSValue,
    a3_type: JSType,
    a3: JSValue,
    a4_type: JSType,
    a4: JSValue,
    a5_type: JSType,
    a5: JSValue,
    a6_type: JSType,
    a6: JSValue,
    a7_type: JSType,
    a7: JSValue,
    a8_type: JSType,
    a8: JSValue,
    a9_type: JSType,
    a9: JSValue,
    a10_type: JSType,
    a10: JSValue,
) -> JSValue {
    unsafe {
        jsfficall10(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9, a10_type, a10,
        )
    }
}

pub fn to_string(c: JSValue) -> alloc::string::String {
    cstr_to_string(c as i32)
}

pub fn to_js_string(s: &str) -> JSValue {
    cstr(s) as JSValue
}


#[no_mangle]
pub fn jsfficallback(
    id: i32,
    a1: JSValue,
    a2: JSValue,
    a3: JSValue,
    a4: JSValue,
    a5: JSValue,
    a6: JSValue,
    a7: JSValue,
    a8: JSValue,
    a9: JSValue,
    a10: JSValue,
) -> () {
    let h = get_callbacks().lock();
    let handler_ref = h.handlers.get(&id).unwrap().clone();
    core::mem::drop(h);
    let handler = handler_ref.lock();
    match &*handler {
        CallbackHandler::Callback0(c) => c(),
        CallbackHandler::Callback1(c) => c(a1),
        CallbackHandler::Callback2(c) => c(a1, a2),
        CallbackHandler::Callback3(c) => c(a1, a2, a3),
        CallbackHandler::Callback4(c) => c(a1, a2, a3, a4),
        CallbackHandler::Callback5(c) => c(a1, a2, a3, a4, a5),
        CallbackHandler::Callback6(c) => c(a1, a2, a3, a4, a5, a6),
        CallbackHandler::Callback7(c) => c(a1, a2, a3, a4, a5, a6, a7),
        CallbackHandler::Callback8(c) => c(a1, a2, a3, a4, a5, a6, a7, a8),
        CallbackHandler::Callback9(c) => c(a1, a2, a3, a4, a5, a6, a7, a8, a9),
        CallbackHandler::Callback10(c) => c(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10),
    }
}

#[no_mangle]
pub fn jsffimalloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}
