#![no_std]
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use spin::Mutex;
extern crate alloc;
#[macro_use]
extern crate lazy_static;
use alloc::vec::Vec;
pub use anystring::anystring;
use cstring::{cstr, cstr_to_string};

pub type JSValue = f64;
pub type JSType = i32;

pub const FALSE: JSValue = 0.0;
pub const TRUE: JSValue = 1.0;

pub const TYPE_NOTHING: JSType = 0;
pub const TYPE_NUMBER: JSType = 1;
pub const TYPE_STRING: JSType = 2;
pub const TYPE_BOOL: JSType = 3;
pub const TYPE_FUNCTION: JSType = 4;
pub const TYPE_OBJECT: JSType = 5;
pub const TYPE_UINT8_ARRAY: JSType = 6;
pub const TYPE_INT8_ARRAY: JSType = 7;
pub const TYPE_UINT8CLAMPED_ARRAY: JSType = 8;
pub const TYPE_INT16_ARRAY: JSType = 9;
pub const TYPE_UINT16_ARRAY: JSType = 10;
pub const TYPE_INT32_ARRAY: JSType = 11;
pub const TYPE_UINT32_ARRAY: JSType = 12;
pub const TYPE_F32_ARRAY: JSType = 13;
pub const TYPE_F64_ARRAY: JSType = 14;
pub const TYPE_BI64_ARRAY: JSType = 15;
pub const TYPE_BUI64_ARRAY: JSType = 16;
pub const TYPE_MEMORY: JSType = 17;

pub const UNDEFINED: JSValue = 0.0;
pub const NULL: JSValue = 1.0;
pub const CONSOLE: JSValue = 2.0;
pub const WINDOW: JSValue = 3.0;
pub const DOCUMENT: JSValue = 4.0;

#[derive(Clone, Copy)]
pub struct JSInvoker(JSValue);

#[macro_export]
macro_rules! js {
    ($($x:tt)*) => {
        {
        register_function(anystring!($($x)*))
        }
    };
}

pub trait ToJSValue {
    #[inline(Always)]
    fn to_js_value(&self) -> JSValue;
    #[inline(Always)]
    fn to_js_type(&self) -> JSType;
}

impl ToJSValue for &str {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        cstr(self) as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_STRING
    }
}

impl ToJSValue for usize {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for u8 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for u16 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for u32 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for i8 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for i16 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for i32 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for bool {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        if *self {
            TRUE as JSValue
        } else {
            FALSE as JSValue
        }
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_BOOL
    }
}

impl ToJSValue for f64 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

impl ToJSValue for f32 {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        *self as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_NUMBER
    }
}

pub struct JSObject(pub JSValue);

impl Drop for JSObject {
    fn drop(&mut self) {
        release_object(self.0)
    }
}

impl ToJSValue for JSObject {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_OBJECT
    }
}

impl ToJSValue for &JSObject {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_OBJECT
    }
}

#[derive(Clone, Copy)]
pub struct JSFunction(JSValue);

impl JSFunction {
    #[inline]
    pub fn from<T>(v: T) -> JSFunction
    where
        T: Clone + Into<f64>,
    {
        JSFunction(v.clone().into() as JSValue)
    }
}

impl ToJSValue for JSFunction {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_FUNCTION
    }
}

#[derive(Clone, Copy)]
pub struct WasmMemory;

impl ToJSValue for WasmMemory {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        0.0
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_MEMORY
    }
}

impl ToJSValue for &Vec<f32> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_F32_ARRAY
    }
}

impl ToJSValue for &Vec<f64> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_F64_ARRAY
    }
}

impl ToJSValue for &Vec<u8> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_UINT8_ARRAY
    }
}

impl ToJSValue for &Vec<i8> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_INT8_ARRAY
    }
}

impl ToJSValue for &Vec<u16> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_UINT16_ARRAY
    }
}

impl ToJSValue for &Vec<i16> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_INT16_ARRAY
    }
}

impl ToJSValue for &Vec<u32> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_UINT32_ARRAY
    }
}

impl ToJSValue for &Vec<i32> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_INT32_ARRAY
    }
}

impl ToJSValue for &Vec<usize> {
    #[inline]
    fn to_js_value(&self) -> JSValue {
        self as *const _ as usize as JSValue
    }

    #[inline]
    fn to_js_type(&self) -> JSType {
        TYPE_UINT32_ARRAY
    }
}

pub trait JSConvert {
    #[inline(Always)]
    fn as_string(&self) -> alloc::string::String;
    #[inline(Always)]
    fn as_owned(&self) -> JSObject;
    #[inline(Always)]
    fn as_bool(&self) -> bool;
    fn as_vec<Q>(&self) -> Vec<Q>
    where
        Q: Copy;
}

impl JSConvert for JSValue {
    #[inline]
    fn as_string(&self) -> alloc::string::String {
        cstr_to_string(*self as i32)
    }

    #[inline]
    fn as_owned(&self) -> JSObject {
        JSObject(*self)
    }

    #[inline]
    fn as_bool(&self) -> bool {
        *self == TRUE
    }

    fn as_vec<Q>(&self) -> Vec<Q>
    where
        Q: Copy,
    {
        unsafe {
            let ptr = *self;
            let start = ptr as usize;
            let lptr = start as *const usize;
            let length = *lptr;
            let mut v = Vec::with_capacity(length);
            let mut data_ptr = ptr as usize + core::mem::size_of::<usize>();
            for _ in 0..length {
                v.push(*(data_ptr as *const Q));
                data_ptr += core::mem::size_of::<Q>();
            }
            v
        }
    }
}

impl JSInvoker {
    #[inline]
    pub fn call_0(&self, obj: impl ToJSValue) -> JSValue {
        unsafe { jsfficall0(obj.to_js_value(), self.0) }
    }

    #[inline]
    pub fn call_1(&self, obj: impl ToJSValue, a1: impl ToJSValue) -> JSValue {
        unsafe { jsfficall1(obj.to_js_value(), self.0, a1.to_js_type(), a1.to_js_value()) }
    }

    #[inline]
    pub fn call_2(&self, obj: impl ToJSValue, a1: impl ToJSValue, a2: impl ToJSValue) -> JSValue {
        unsafe {
            jsfficall2(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_3(
        &self,
        obj: impl ToJSValue,
        a1: impl ToJSValue,
        a2: impl ToJSValue,
        a3: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall3(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_4(
        &self,
        obj: impl ToJSValue,
        a1: impl ToJSValue,
        a2: impl ToJSValue,
        a3: impl ToJSValue,

        a4: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall4(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_5(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall5(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_6(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall6(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_7(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall7(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_8(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall8(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_9(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,

        a9: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall9(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
                a9.to_js_type(),
                a9.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn call_10(
        &self,
        obj: impl ToJSValue,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,

        a9: impl ToJSValue,

        a10: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall10(
                obj.to_js_value(),
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
                a9.to_js_type(),
                a9.to_js_value(),
                a10.to_js_type(),
                a10.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_0(&self) -> JSValue {
        unsafe { jsfficall0(UNDEFINED, self.0) }
    }

    #[inline]
    pub fn invoke_1(&self, a1: impl ToJSValue) -> JSValue {
        unsafe { jsfficall1(UNDEFINED, self.0, a1.to_js_type(), a1.to_js_value()) }
    }

    #[inline]
    pub fn invoke_2(&self, a1: impl ToJSValue, a2: impl ToJSValue) -> JSValue {
        unsafe {
            jsfficall2(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_3(&self, a1: impl ToJSValue, a2: impl ToJSValue, a3: impl ToJSValue) -> JSValue {
        unsafe {
            jsfficall3(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_4(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall4(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_5(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall5(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_6(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall6(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_7(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall7(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_8(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall8(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_9(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,

        a9: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall9(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
                a9.to_js_type(),
                a9.to_js_value(),
            )
        }
    }

    #[inline]
    pub fn invoke_10(
        &self,

        a1: impl ToJSValue,

        a2: impl ToJSValue,

        a3: impl ToJSValue,

        a4: impl ToJSValue,

        a5: impl ToJSValue,

        a6: impl ToJSValue,

        a7: impl ToJSValue,

        a8: impl ToJSValue,

        a9: impl ToJSValue,

        a10: impl ToJSValue,
    ) -> JSValue {
        unsafe {
            jsfficall10(
                UNDEFINED,
                self.0,
                a1.to_js_type(),
                a1.to_js_value(),
                a2.to_js_type(),
                a2.to_js_value(),
                a3.to_js_type(),
                a3.to_js_value(),
                a4.to_js_type(),
                a4.to_js_value(),
                a5.to_js_type(),
                a5.to_js_value(),
                a6.to_js_type(),
                a6.to_js_value(),
                a7.to_js_type(),
                a7.to_js_value(),
                a8.to_js_type(),
                a8.to_js_value(),
                a9.to_js_type(),
                a9.to_js_value(),
                a10.to_js_type(),
                a10.to_js_value(),
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

pub fn register_function(code: &str) -> JSInvoker {
    unsafe { JSInvoker(jsffiregister(cstr(code))) }
}

#[no_mangle]
fn jsfficallback(
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
fn jsffimalloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

enum CallbackHandler {
    Callback0(Box<dyn FnMut() -> () + Send + 'static>),
    Callback1(Box<dyn FnMut(JSValue) -> () + Send + 'static>),
    Callback2(Box<dyn FnMut(JSValue, JSValue) -> () + Send + 'static>),
    Callback3(Box<dyn FnMut(JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback4(Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback5(Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>),
    Callback6(
        Box<dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
    ),
    Callback7(
        Box<
            dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback8(
        Box<
            dyn FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback9(
        Box<
            dyn FnMut(
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                ) -> ()
                + Send
                + 'static,
        >,
    ),
    Callback10(
        Box<
            dyn FnMut(
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                    JSValue,
                ) -> ()
                + Send
                + 'static,
        >,
    ),
}

type CallbackHandle = u32;

struct CallbackManager {
    cur_id: CallbackHandle,
    keys: Vec<CallbackHandle>,
    handlers: Vec<Arc<Mutex<CallbackHandler>>>,
}

lazy_static! {
    static ref INSTANCE: Mutex<CallbackManager> = {
        Mutex::new(CallbackManager {
            cur_id: 0,
            keys: Vec::new(),
            handlers: Vec::new(),
        })
    };
}

fn get_callbacks() -> &'static Mutex<CallbackManager> {
    &INSTANCE
}

fn get_callback(id: CallbackHandle) -> Option<Arc<Mutex<CallbackHandler>>> {
    let cbs = get_callbacks().lock();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        let handler_ref = cbs.handlers.get(i).unwrap().clone();
        core::mem::drop(cbs);
        Some(handler_ref)
    } else {
        None
    }
}

pub fn remove_callback(id: CallbackHandle) {
    let mut cbs = get_callbacks().lock();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        cbs.keys.remove(i);
        cbs.handlers.remove(i);
    }
}

fn create_callback(cb: CallbackHandler) -> JSFunction {
    let mut h = get_callbacks().lock();
    h.cur_id += 1;
    let id = h.cur_id;
    h.keys.push(id);
    h.handlers.push(Arc::new(Mutex::new(cb)));
    return JSFunction::from(id);
}

pub fn create_callback_0(cb: impl FnMut() -> () + Send + 'static) -> JSFunction {
    create_callback(CallbackHandler::Callback0(Box::new(cb)))
}

pub fn create_callback_1(cb: impl FnMut(JSValue) -> () + Send + 'static) -> JSFunction {
    create_callback(CallbackHandler::Callback1(Box::new(cb)))
}

pub fn create_callback_2(cb: impl FnMut(JSValue, JSValue) -> () + Send + 'static) -> JSFunction {
    create_callback(CallbackHandler::Callback2(Box::new(cb)))
}

pub fn create_callback_3(
    cb: impl FnMut(JSValue, JSValue, JSValue) -> () + Send + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback3(Box::new(cb)))
}

pub fn create_callback_4(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback4(Box::new(cb)))
}

pub fn create_callback_5(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback5(Box::new(cb)))
}

pub fn create_callback_6(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback6(Box::new(cb)))
}

pub fn create_callback_7(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback7(Box::new(cb)))
}

pub fn create_callback_8(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
        + Send
        + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback8(Box::new(cb)))
}

pub fn create_callback_9(
    cb: impl FnMut(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
        + Send
        + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback9(Box::new(cb)))
}

pub fn create_callback_10(
    cb: impl FnMut(
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
            JSValue,
        ) -> ()
        + Send
        + 'static,
) -> JSFunction {
    create_callback(CallbackHandler::Callback10(Box::new(cb)))
}

struct CallbackFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
    result: JSValue,
}

impl Future for CallbackFuture {
    type Output = JSValue;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture {
    fn new() -> (Self, JSFunction) {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
            result: UNDEFINED,
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback1(Box::new(move |v: JSValue| {
            let mut shared_state = thread_shared_state.lock();
            shared_state.completed = true;
            shared_state.result = v;
            if let Some(waker) = shared_state.waker.take() {
                core::mem::drop(shared_state);
                waker.wake()
            }
        })));
        (CallbackFuture { shared_state }, id)
    }
}

pub fn create_callback_future_0() -> (impl Future, JSFunction) {
    CallbackFuture::new()
}
