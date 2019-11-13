#![no_std]
pub use anystring::anystring;
extern crate alloc;
use alloc::vec::Vec;
pub use callback::*;
use cstring::{cstr, cstr_to_string};
pub use web_common::*;

#[derive(Clone, Copy)]
pub struct JSFunction(JSValue);

#[macro_export]
macro_rules! js {
    ($($x:tt)*) => {
        {
        register_function(anystring!($($x)*))
        }
    };
}

impl JSFunction {
    pub fn call_0(&self, obj: JSValue) -> JSValue {
        unsafe { jsfficall0(obj, self.0) }
    }

    pub fn call_1(&self, obj: JSValue, a1_type: JSType, a1: JSValue) -> JSValue {
        unsafe { jsfficall1(obj, self.0, a1_type, a1) }
    }

    pub fn call_2(
        &self,
        obj: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
    ) -> JSValue {
        unsafe { jsfficall2(obj, self.0, a1_type, a1, a2_type, a2) }
    }

    pub fn call_3(
        &self,
        obj: JSValue,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
    ) -> JSValue {
        unsafe { jsfficall3(obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3) }
    }

    pub fn call_4(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4,
            )
        }
    }

    pub fn call_5(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            )
        }
    }

    pub fn call_6(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6,
            )
        }
    }

    pub fn call_7(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7,
            )
        }
    }

    pub fn call_8(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8,
            )
        }
    }

    pub fn call_9(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9,
            )
        }
    }

    pub fn call_10(
        &self,
        obj: JSValue,
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
                obj, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9, a10_type, a10,
            )
        }
    }

    pub fn invoke_0(&self) -> JSValue {
        unsafe { jsfficall0(UNDEFINED, self.0) }
    }

    pub fn invoke_1(&self, a1_type: JSType, a1: JSValue) -> JSValue {
        unsafe { jsfficall1(UNDEFINED, self.0, a1_type, a1) }
    }

    pub fn invoke_2(&self, a1_type: JSType, a1: JSValue, a2_type: JSType, a2: JSValue) -> JSValue {
        unsafe { jsfficall2(UNDEFINED, self.0, a1_type, a1, a2_type, a2) }
    }

    pub fn invoke_3(
        &self,
        a1_type: JSType,
        a1: JSValue,
        a2_type: JSType,
        a2: JSValue,
        a3_type: JSType,
        a3: JSValue,
    ) -> JSValue {
        unsafe { jsfficall3(UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3) }
    }

    pub fn invoke_4(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4,
            )
        }
    }

    pub fn invoke_5(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            )
        }
    }

    pub fn invoke_6(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6,
            )
        }
    }

    pub fn invoke_7(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7,
            )
        }
    }

    pub fn invoke_8(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8,
            )
        }
    }

    pub fn invoke_9(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9,
            )
        }
    }

    pub fn invoke_10(
        &self,
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
                UNDEFINED, self.0, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
                a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9, a10_type, a10,
            )
        }
    }
}

type CStrPtr = i32;

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

pub fn release_object(obj: JSValue) {
    unsafe { jsffirelease(obj) }
}

pub fn register_function(code: &str) -> JSFunction {
    unsafe { JSFunction(jsffiregister(cstr(code))) }
}

pub fn to_string(c: JSValue) -> alloc::string::String {
    cstr_to_string(c as i32)
}

pub fn to_js_string(s: &str) -> JSValue {
    cstr(s) as JSValue
}

pub fn to_js_typed_array<T>(s: &[T]) -> TypedArray {
    TypedArray {
        length: s.len() as u32,
        pointer: s.as_ptr() as u32,
    }
}

pub fn to_typed_array<T>(ptr: JSValue) -> Vec<T>
where
    T: Copy,
{
    unsafe {
        let start = ptr as usize;
        let lptr = start as *const usize;
        let length = *lptr;
        let mut v = Vec::with_capacity(length);
        let mut data_ptr = ptr as usize + core::mem::size_of::<usize>();
        for _ in 0..length {
            v.push(*(data_ptr as *const T));
            data_ptr += core::mem::size_of::<T>();
        }
        v
    }
}

#[repr(C)]
pub struct TypedArray {
    length: u32,
    pointer: u32,
}

impl TypedArray {
    pub fn as_js_ptr(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }
}

#[no_mangle]
pub fn jsfficallback(
    id: u32,
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
    let h = get_callback(id);
    let handler_ref = h.unwrap().clone();
    let mut handler = handler_ref.lock();
    match &mut *handler {
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
