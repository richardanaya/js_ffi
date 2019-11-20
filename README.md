# js_ffi

<a href="https://docs.rs/js_ffi"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A foreign function interface(FFI) library for invoking Javascript functions from Web Assembly with Rust

- [x] no code generation or special cargo components
- [x] support for callbacks (e.g. `setTimeout`)
- [x] futures based on callbacks
- [x] memory as a parameter
- [x] works with web assembly languages other than Rust
- [x] a `js!` macro for inline javascript
- [x] typed arrays
- [ ] usable with nodejs

This project has similaries to Javascript's `<function>.call(<object>,a0,a1,...)` but with the limitations of Web Assembly's function call restrictions.

## Hello World!
```toml
[dependencies]
js_ffi = "0.6"
```
```rust
use js_ffi::*;
​
#[no_mangle]
pub fn main() -> () {
    js!(console.log).invoke_1("Hello World");
}
```
```html
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>js_ffi.run("example.wasm");</script>
```

## How it works

1. Get a handle to some Javascript function using the `js!` macro. Re-use this handle as often as possible.
2. If you are invoking this function as a regular function, use the appropriate `invoke_*` function based on the number of arguments you are passing (`invoke_1`,`invoke_7`,etc.).
3. If you are invoking this function as a method of an object represented by a `JSValue`, use the appropriate `call_*` function based on the number of arguments you are passing (`call_1`,`invoke_7`,etc.) and make sure your object is the first paramter.

## Event Listener

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    let btn = js!(document.querySelector).call_1(DOCUMENT, "#button");
    js!(Node.prototype.addEventListener).call_2(
        btn,
        "click",
        create_callback_0(|| {
            js!(window.alert).invoke_1("I was clicked");
        }),
    );
}
```

## Async Example

Using an [`executor`](https://www.github.com/richardanaya/executor) library we can easily turn callbacks into futures and run behavior asynchronously.

```rust
use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    executor::spawn(async {
        let console_log = js!(console.log);
        console_log.invoke_1("Hello");
        sleep(1000).await;
        console_log.invoke_1("world!");
    });
}

fn sleep(millis:u32) {
    let set_timeout = js!(window.setTimeout);
    let (future, cb) = create_callback_future_0();
    set_timeout.invoke_2(cb,1000);
    future.await;
}
```

## Third Party

Wrap third party libraries. Anything function in global space should be able to be wrapped and invoked.

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    // register functions of things in global scope
    let my_fn = js!((x) => { 
        console.log("say something here too");
        say_loud(x);
    });
    my_fn.invoke_1("hey");
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
for inclusion in `js_ffi` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
