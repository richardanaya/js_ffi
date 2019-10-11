#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use cstring::{cstr, cstr_to_string};
use hashbrown::HashMap;
use once_cell::sync::OnceCell;
use spin::Mutex;

pub type FunctionHandle = i32;
pub type JSValue = f32;

extern "C" {
    fn jsffirelease(obj: JSValue);
    fn jsffiregister(code: i32) -> FunctionHandle;
    fn jsfficall0(obj: JSValue, function: i32) -> JSValue;
    fn jsfficall1(obj: JSValue, function: i32, a1_type: i32, a1: JSValue) -> JSValue;
    fn jsfficall2(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
    ) -> JSValue;
    fn jsfficall3(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
    ) -> JSValue;
    fn jsfficall4(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
    ) -> JSValue;
    fn jsfficall5(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
    ) -> JSValue;
    fn jsfficall6(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
        a6_type: i32,
        a6: JSValue,
    ) -> JSValue;
    fn jsfficall7(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
        a6_type: i32,
        a6: JSValue,
        a7_type: i32,
        a7: JSValue,
    ) -> JSValue;
    fn jsfficall8(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
        a6_type: i32,
        a6: JSValue,
        a7_type: i32,
        a7: JSValue,
        a8_type: i32,
        a8: JSValue,
    ) -> JSValue;
    fn jsfficall9(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
        a6_type: i32,
        a6: JSValue,
        a7_type: i32,
        a7: JSValue,
        a8_type: i32,
        a8: JSValue,
        a9_type: i32,
        a9: JSValue,
    ) -> JSValue;
    fn jsfficall10(
        obj: JSValue,
        function: i32,
        a1_type: i32,
        a1: JSValue,
        a2_type: i32,
        a2: JSValue,
        a3_type: i32,
        a3: JSValue,
        a4_type: i32,
        a4: JSValue,
        a5_type: i32,
        a5: JSValue,
        a6_type: i32,
        a6: JSValue,
        a7_type: i32,
        a7: JSValue,
        a8_type: i32,
        a8: JSValue,
        a9_type: i32,
        a9: JSValue,
        a10_type: i32,
        a10: JSValue,
    ) -> JSValue;
}

pub const UNDEFINED: JSValue = 0.0;
pub const NULL: JSValue = 1.0;
pub const CONSOLE: JSValue = 2.0;
pub const WINDOW: JSValue = 3.0;
pub const DOCUMENT: JSValue = 4.0;

pub const FALSE: JSValue = 0.0;
pub const TRUE: JSValue = 1.0;

pub const TYPE_NOTHING: i32 = 0;
pub const TYPE_NUM: i32 = 1;
pub const TYPE_STRING: i32 = 2;
pub const TYPE_BOOL: i32 = 3;
pub const TYPE_FUNCTION: i32 = 4;
pub const TYPE_OBJECT: i32 = 5;

pub fn release(obj: JSValue) {
    unsafe { jsffirelease(obj) }
}

pub fn register(code: &str) -> FunctionHandle {
    unsafe { jsffiregister(cstr(code)) }
}

pub fn call_0(obj: JSValue, function: i32) -> JSValue {
    unsafe { jsfficall0(obj, function) }
}

pub fn call_1(obj: JSValue, function: i32, a1_type: i32, a1: JSValue) -> JSValue {
    unsafe { jsfficall1(obj, function, a1_type, a1) }
}

pub fn call_2(
    obj: JSValue,
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: JSValue,
) -> JSValue {
    unsafe { jsfficall2(obj, function, a1_type, a1, a2_type, a2) }
}

pub fn call_3(
    obj: JSValue,
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
) -> JSValue {
    unsafe { jsfficall3(obj, function, a1_type, a1, a2_type, a2, a3_type, a3) }
}

pub fn call_4(
    obj: JSValue,
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
    a5: JSValue,
    a6_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
    a5: JSValue,
    a6_type: i32,
    a6: JSValue,
    a7_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
    a5: JSValue,
    a6_type: i32,
    a6: JSValue,
    a7_type: i32,
    a7: JSValue,
    a8_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
    a5: JSValue,
    a6_type: i32,
    a6: JSValue,
    a7_type: i32,
    a7: JSValue,
    a8_type: i32,
    a8: JSValue,
    a9_type: i32,
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
    function: i32,
    a1_type: i32,
    a1: JSValue,
    a2_type: i32,
    a2: f32,
    a3_type: i32,
    a3: JSValue,
    a4_type: i32,
    a4: JSValue,
    a5_type: i32,
    a5: JSValue,
    a6_type: i32,
    a6: JSValue,
    a7_type: i32,
    a7: JSValue,
    a8_type: i32,
    a8: JSValue,
    a9_type: i32,
    a9: JSValue,
    a10_type: i32,
    a10: JSValue,
) -> JSValue {
    unsafe {
        jsfficall10(
            obj, function, a1_type, a1, a2_type, a2, a3_type, a3, a4_type, a4, a5_type, a5,
            a6_type, a6, a7_type, a7, a8_type, a8, a9_type, a9, a10_type, a10,
        )
    }
}

pub enum CallbackHandler {
    Callback0(Box<dyn Fn() -> () + Send + 'static>),
    Callback1(Box<dyn Fn(f32) -> () + Send + 'static>),
    Callback2(Box<dyn Fn(f32, f32) -> () + Send + 'static>),
    Callback3(Box<dyn Fn(f32, f32, f32) -> () + Send + 'static>),
    Callback4(Box<dyn Fn(f32, f32, f32, f32) -> () + Send + 'static>),
    Callback5(Box<dyn Fn(f32, f32, f32, f32, f32) -> () + Send + 'static>),
    Callback6(Box<dyn Fn(f32, f32, f32, f32, f32, f32) -> () + Send + 'static>),
    Callback7(Box<dyn Fn(f32, f32, f32, f32, f32, f32, f32) -> () + Send + 'static>),
    Callback8(Box<dyn Fn(f32, f32, f32, f32, f32, f32, f32, f32) -> () + Send + 'static>),
    Callback9(Box<dyn Fn(f32, f32, f32, f32, f32, f32, f32, f32, f32) -> () + Send + 'static>),
    Callback10(
        Box<dyn Fn(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) -> () + Send + 'static>,
    ),
}

struct Callback {
    cur_id: i32,
    handlers: HashMap<i32, Arc<Mutex<CallbackHandler>>>,
}

pub fn to_string(c: JSValue) -> alloc::string::String {
    cstr_to_string(c as i32)
}

pub fn to_js_string(s: &str) -> JSValue {
    cstr(s) as f32
}

fn get_callbacks() -> &'static Mutex<Callback> {
    static INSTANCE: OnceCell<Mutex<Callback>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(Callback {
            cur_id: 0,
            handlers: HashMap::new(),
        })
    })
}

fn create_callback(cb: CallbackHandler) -> JSValue {
    let mut h = get_callbacks().lock();
    h.cur_id += 1;
    let id = h.cur_id;
    h.handlers.insert(id, Arc::new(Mutex::new(cb)));
    return id as f32;
}

pub fn create_callback0(cb: Box<dyn Fn() -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback0(cb))
}

pub fn create_callback1(cb: Box<dyn Fn(JSValue) -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback1(cb))
}

pub fn create_callback2(cb: Box<dyn Fn(JSValue, JSValue) -> () + Send + 'static>) -> JSValue {
    create_callback(CallbackHandler::Callback2(cb))
}

pub fn create_callback3(
    cb: Box<dyn Fn(JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback3(cb))
}

pub fn create_callback4(
    cb: Box<dyn Fn(JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback4(cb))
}

pub fn create_callback5(
    cb: Box<dyn Fn(JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback5(cb))
}

pub fn create_callback6(
    cb: Box<dyn Fn(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> () + Send + 'static>,
) -> JSValue {
    create_callback(CallbackHandler::Callback6(cb))
}

pub fn create_callback7(
    cb: Box<
        dyn Fn(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback7(cb))
}

pub fn create_callback8(
    cb: Box<
        dyn Fn(JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue, JSValue) -> ()
            + Send
            + 'static,
    >,
) -> JSValue {
    create_callback(CallbackHandler::Callback8(cb))
}

pub fn create_callback9(
    cb: Box<
        dyn Fn(
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
) -> JSValue {
    create_callback(CallbackHandler::Callback9(cb))
}

pub fn create_callback10(
    cb: Box<
        dyn Fn(
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
) -> JSValue {
    create_callback(CallbackHandler::Callback10(cb))
}

pub struct CallbackFuture {
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
    pub fn new() -> (Self, f32) {
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
        (CallbackFuture { shared_state }, id as f32)
    }
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
