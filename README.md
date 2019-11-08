# js_ffi

```toml
[dependencies]
js_ffi = "0.0.14"
```

A simple FFI library for invoking javascript functions from web assembly with Rust
* support for callbacks, typed arrays, javascript function invocations, and references to javascript objects
* flexible enough to work with nodejs and apis beyond the stand browser apis
* works with web assembly languages other than Rust

Think of it like a Rust version of javascript's `<function>.call(<object>,a0,a1,...)` but limited by web assembly's function call restrictions.

## Hello World!

```rust
// get a reference to the function `console.log` in javascript
let log:JSValue = register("console.log");
// call the function with 1 string parameter
call_1(UNDEFINED, log, TYPE_STRING, to_js_string("Hello World"));
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

```html
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>js_ffi.run("example.wasm");</script>
```

## How it works

The basic premise is that you `register` the JavaScript functions you want to have access to from Rust to a constant number function handle. Then you can use `call_*` to send execute the function with arguments depending on the argument count you  want to send (e.g. `call_1`, `call_7`). The idea is you can quickly create wrapper functions for exactly what you need. When calling the function you specify the object to call the function of (or undefined if you just want to call the function), the function id to call you registered with, and pairs of argument type and arguments afer.

`call_*(<object handle>,<function id>,<arg type>,<arg>,<arg type>,<arg>,...)`

## Advanced

Wrap third party. Anything with its functions in global space should be able to be wrapped and invoked.

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let jquery_handle = register("$");
    // someimes functions are hidden on prototypes of things in global scope
    let jquery_on_handle = register("jQuery.prototype.on");
    // reference your own functions created in global scope
    let alert = register("(msg)=>window.alert(msg)");

    let obj = call_1(UNDEFINED, jquery_handle, TYPE_STRING, to_js_string("body"));
    call_2(
        obj,
        jquery_on_handle,
        TYPE_STRING,
        to_js_string("click"),
        TYPE_FUNCTION,
        create_callback1(Box::new(move |_| {
            call_1(
                UNDEFINED,
                alert,
                TYPE_STRING,
                to_js_string("I was clicked!"),
            );
        })),
    );
}
```

```html
<script src="https://code.jquery.com/jquery-3.4.1.min.js"></script>
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>
    function say_loud(msg){
        window.alert(msg);
    }
</script>
<script>js_ffi.run("example.wasm");</script>
```

# Don't like Rust?

The script `js_ffi.js` has nothing Rust specific.  Everything is only done through a very basic interface

* `jsffiregister(i32)->i32`
* `jsffirelease(i32)`
* `jsfficall0(f32,i32)->f32`
* `jsfficall1(f32,i32,i32,f32)->f32`
* `jsfficall2(f32,i32,i32,f32,i32,f32)->f32`
* ...
* `jsfficall10(f32,i32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32,i32,f32)->f32`

an entry point function:

* `main()`

and expects on your module:

* jsffimalloc(i32) -> i32
* jsfficallback(i32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32)

As long as your module adheres to this you can use `js_ffi`. Strings are simply c-strings in memorythat end in a `0` character.

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
