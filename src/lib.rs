#![no_std]
extern crate alloc;
#[macro_use]
extern crate lazy_static;
use alloc::vec::Vec;
pub use anystring::anystring;
pub use callback::*;
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

#[derive(Clone)]
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
    fn to_js_value(&mut self) -> JSValue;
    #[inline(Always)]
    fn to_js_type(&mut self) -> JSType;
}

#[derive(Clone)]
pub struct JSGlobal(pub JSValue);

impl JSGlobal {
    pub fn document() -> JSGlobal {
        JSGlobal(DOCUMENT)
    }

    pub fn undefined() -> JSGlobal {
        JSGlobal(UNDEFINED)
    }

    pub fn null() -> JSGlobal {
        JSGlobal(NULL)
    }

    pub fn window() -> JSGlobal {
        JSGlobal(WINDOW)
    }
}

impl ToJSValue for JSGlobal {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_OBJECT
    }
}

impl ToJSValue for &JSGlobal {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_OBJECT
    }
}

pub struct JSObject(pub JSValue);

impl JSObject {
    pub fn document() -> JSObject {
        JSObject(DOCUMENT)
    }
}

impl Drop for JSObject {
    fn drop(&mut self) {
        release_object(self.0)
    }
}

impl ToJSValue for JSObject {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_OBJECT
    }
}

impl ToJSValue for &JSObject {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_OBJECT
    }
}

#[derive(Clone)]
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
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_FUNCTION
    }
}

#[derive(Clone)]
pub struct JSBool(JSValue);

impl JSBool {
    #[inline]
    pub fn from(b: bool) -> JSBool {
        JSBool(if b { TRUE } else { FALSE })
    }
}

impl ToJSValue for JSBool {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_BOOL
    }
}

#[derive(Clone)]
pub struct JSNumber(JSValue);

impl JSNumber {
    #[inline]
    pub fn from<T>(v: T) -> JSNumber
    where
        T: Clone + Into<f64>,
    {
        JSNumber(v.clone().into() as JSValue)
    }
}

impl ToJSValue for JSNumber {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        self.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_NUMBER
    }
}

#[derive(Clone)]
pub struct WasmMemory;

impl ToJSValue for WasmMemory {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        0.0
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_MEMORY
    }
}

#[derive(Clone)]
pub struct JSTypedArray<'t, T> {
    data: &'t Vec<T>,
    snapshot: Option<TypedArray>,
}

impl<'t, T> JSTypedArray<'t, T> {
    #[inline]
    pub fn from(s: &'t Vec<T>) -> JSTypedArray<'t, T> {
        JSTypedArray {
            data: s,
            snapshot: None,
        }
    }
}

impl<'t, T> ToJSValue for JSTypedArray<'t, T>
where
    T: 'static,
{
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        let s = to_js_typed_array(self.data);
        self.snapshot = Some(s);
        self.snapshot.as_mut().unwrap().to_js_value()
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        if core::any::TypeId::of::<T>() == core::any::TypeId::of::<f32>() {
            TYPE_F32_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<f64>() {
            TYPE_F64_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u8>() {
            TYPE_UINT8_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<i8>() {
            TYPE_INT8_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u16>() {
            TYPE_UINT16_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<i16>() {
            TYPE_INT16_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<i32>() {
            TYPE_INT32_ARRAY
        } else if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u32>() {
            TYPE_UINT32_ARRAY
        } else {
            panic!("unsupported typed array type")
        }
    }
}

#[derive(Clone)]
pub struct JSString<'t> {
    data: &'t str,
}

impl<'t> JSString<'t> {
    #[inline]
    pub fn from(s: &'t str) -> JSString<'t> {
        JSString { data: s }
    }
}

impl<'t> ToJSValue for JSString<'t> {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
        to_js_string(self.data)
    }

    #[inline]
    fn to_js_type(&mut self) -> JSType {
        TYPE_STRING
    }
}

impl JSInvoker {
    #[inline]
    pub fn call_0(&self, mut obj: impl ToJSValue) -> JSValue {
        unsafe { jsfficall0(obj.to_js_value(), self.0) }
    }

    #[inline]
    pub fn call_1(&self, mut obj: impl ToJSValue, mut a1: impl ToJSValue) -> JSValue {
        unsafe { jsfficall1(obj.to_js_value(), self.0, a1.to_js_type(), a1.to_js_value()) }
    }

    #[inline]
    pub fn call_2(
        &self,
        mut obj: impl ToJSValue,
        mut a1: impl ToJSValue,
        mut a2: impl ToJSValue,
    ) -> JSValue {
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
        mut obj: impl ToJSValue,
        mut a1: impl ToJSValue,
        mut a2: impl ToJSValue,
        mut a3: impl ToJSValue,
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
        mut obj: impl ToJSValue,
        mut a1: impl ToJSValue,
        mut a2: impl ToJSValue,
        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,

        mut a9: impl ToJSValue,
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
        mut obj: impl ToJSValue,

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,

        mut a9: impl ToJSValue,

        mut a10: impl ToJSValue,
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
    pub fn invoke_1(&self, mut a1: impl ToJSValue) -> JSValue {
        unsafe { jsfficall1(UNDEFINED, self.0, a1.to_js_type(), a1.to_js_value()) }
    }

    #[inline]
    pub fn invoke_2(&self, mut a1: impl ToJSValue, mut a2: impl ToJSValue) -> JSValue {
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
    pub fn invoke_3(
        &self,
        mut a1: impl ToJSValue,
        mut a2: impl ToJSValue,
        mut a3: impl ToJSValue,
    ) -> JSValue {
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,

        mut a9: impl ToJSValue,
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

        mut a1: impl ToJSValue,

        mut a2: impl ToJSValue,

        mut a3: impl ToJSValue,

        mut a4: impl ToJSValue,

        mut a5: impl ToJSValue,

        mut a6: impl ToJSValue,

        mut a7: impl ToJSValue,

        mut a8: impl ToJSValue,

        mut a9: impl ToJSValue,

        mut a10: impl ToJSValue,
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
        data_type: TYPE_F32_ARRAY,
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
#[derive(Clone)]
pub struct TypedArray {
    length: u32,
    pointer: u32,
    data_type: JSType,
}

impl TypedArray {
    #[inline]
    fn to_js_value(&mut self) -> JSValue {
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

mod callback;

pub use callback::*;
