# js_ffi

<a href="https://docs.rs/js_ffi"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple FFI library for invoking javascript functions from web assembly with Rust
* support for callbacks, callbacks as futures, typed arrays, javascript function invocations, and references to javascript objects
* flexible enough to work with nodejs and apis beyond the stand browser apis
* works with web assembly languages other than Rust

Think of it like a Rust version of javascript's `<function>.call(<object>,a0,a1,...)` but limited by web assembly's function call restrictions.

[Documentation](https://docs.rs/js_ffi/)

## How it works

1. `register` the javascript function to and get a `JSValue` handle to it
2. use the correct function to call that function based on number of args you are sending (`call_0`,`call_1`,etc.)
3. if you are calling the function as a method on object represented by a `JSValue` you already have, pass it as the first parameter
4. for each argument specify the type of the argument (`TYPE_STRING`,`TYPE_NUMBER`, etc.)

## Hello World!
```toml
[dependencies]
js_ffi = "0.1.0"
```
```rust
// get a reference to the function `console.log` in javascript
let log:JSValue = register("console.log");
// call the function with 1 string parameter
call_1(UNDEFINED, log, TYPE_STRING, to_js_string("Hello World"));
```
```html
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>js_ffi.run("example.wasm");</script>
```

## Event Listener

```rust
// register the methods we want to use
let query_selector = register("document.querySelector");
let add_event_listener = register("Node.prototype.addEventListener");
let alert = register("window.alert");

// this call returns a JSValue that is a reference to the button
let btn = call_1(UNDEFINED, query_selector, TYPE_STRING, to_js_string("#button"));

// creating a callback returns a JSValue reference to the callback function
// note: this is specifically the number of paramters when creating callbacks
let cb = create_callback_0(Box::new(||{
    call_1(UNDEFINED, alert, TYPE_STRING, to_js_string("I was clicked"));
}));

// since we are calling `addEventListener` as a method of `btn`, we pass it 
// in the first parameter as the object context
call_2(btn, add_event_listener, TYPE_STRING, to_js_string("click"),TYPE_FUNCTION,cb)
```

## Async Example

```rust
use executor::Executor;
use js_ffi::*;

async fn run() {
    let console_log = register("console.log");
    let set_timeout = register("window.setTimeout");
    log(console_log,"hello")
    window_set_timeout(set_timeout,1000).await;
    log(console_log,"hello")
}

#[no_mangle]
pub fn main() -> () {
    Executor::spawn(run());
}

pub fn console_log(fn_ref:JSValue, msg: &str) {
    call_1(UNDEFINED,fn_ref,TYPE_STRING,to_js_string(msg));
}

pub fn window_set_timeout(fn_ref:JSValue, millis: i32) -> CallbackFuture {
    let (future, id) = CallbackFuture::new();
    call_2(UNDEFINED,fn_ref,TYPE_FUNCTION,id,TYPE_NUM,millis as JSValue);
    future
}
```

## Third Party

Wrap third party libraries. Anything function in global space should be able to be wrapped and invoked. You can also specify ad hoc functions.

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let my_fn = register("(x) => { 
        console.log("say something here too");
        say_loud(x);
    }");
    call_0(UNDEFINED,my_fn,TYPE_STRING,"hey");
}
```

```html
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>
    function say_loud(msg){
        window.alert(msg);
    }
</script>
<script>js_ffi.run("example.wasm");</script>
```

# Don't like Rust?

The script `js_ffi.js` has nothing Rust specific.  

* Operations execute through an interface specified in this [`js_ffi.h`](https://github.com/richardanaya/js_ffi/blob/master/js_ffi.h)
* `js_ffi` expects an entry point `main()`
* If you plan on having your module receive data it must implement `jsffimalloc(i32) -> i32`
* If you plan on having your module receive callbacks it must implement `jsfficallback(i32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32)`
* strings are simply c-strings in memory that end in a `0` character.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in js_ffi by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
