# js_ffi

**this library creates a bridge to javascript in the browser at runtime using web assembly**

<a href="https://docs.rs/js_ffi"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A foreign function interface(FFI) library for invoking Javascript functions from Web Assembly for many programming languages

- [x] no code generation or special cargo components
- [x] support for callbacks (e.g. `setTimeout`)
- [x] futures based on callbacks
- [x] memory as a parameter
- [x] wrapper library for Rust
- [x] works with C or C++, [check out examples here](https://github.com/richardanaya/js_ffi/tree/master/examples_in_c)
- [x] a `js!` macro for inline javascript
- [x] typed arrays
- [x] can be executed in a web worker

This project has similarities to Javascript's `<function>.call(<object>,a0,a1,...)` but with the limitations of Web Assembly's function call restrictions.

## Hello World! in Rust

**note js_ffi is language agnostic, I just used Rust as example because I like it**

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
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi@latest/js_ffi.js"></script>
<script>js_ffi.run("example.wasm");</script>
```
```makefile
# cli commands for building web assembly
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/helloworld.wasm .
lint:
	@cargo fmt
serve:
	python3 -m http.server 8080
```


# Drawing

See demo [here](https://richardanaya.github.io/js_ffi/examples/canvas/)

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    let screen = js!(document.querySelector).call_1(&DOCUMENT, "#screen").to_js_object();
    let ctx = js!(document.querySelector).call_1(&screen, "#screen").to_js_object();

    let fill_style = js!(function(color){
        this.fillStyle = color;
    });
    let fill_rect = js!(CanvasRenderingContext2D.prototype.fillRect);

    fill_style.call_1(&ctx, "red");
    fill_rect.call_4(&ctx, 0.0, 0.0, 50.0, 50.0);

    fill_style.call_1(&ctx, "green");
    fill_rect.call_4(&ctx, 15.0, 15.0, 50.0, 50.0);

    fill_style.call_1(&ctx, "blue");
    fill_rect.call_4(&ctx, 30.0, 30.0, 50.0, 50.0);
}
```

## Event Listener

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    let btn = js!(document.querySelector).call_1(&DOCUMENT, "#button").to_js_object();
    js!(Node.prototype.addEventListener).call_2(
        &btn,
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

fn sleep(millis: u32) -> impl core::future::Future {
    let set_timeout = js!(window.setTimeout);
    let (future, cb) = create_callback_future_0();
    set_timeout.invoke_2(cb, millis);
    future
}
```

## Third Party

Wrap third party libraries. Anything function in global space should be able to be wrapped and invoked.

```rust
use js_ffi::*;

#[no_mangle]
fn main() {
    let jquery_handle = js!($);
    let jquery_on_handle = js!(jQuery.prototype.on);
    let alert = js!((msg)=>window.alert(msg));

    let body = jquery_handle.invoke_1("body").to_js_object();
    jquery_on_handle.call_2(
        &body,
        "click",
        create_callback_1(move |_event| {
            alert.invoke_1("I was clicked!");
        }),
    );
}
```

```html
<script src="https://code.jquery.com/jquery-3.4.1.js"></script>
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>js_ffi.run("example.wasm");</script>
```

# Standard Web Libraries

A collection of libraries exist that expose javascript functionality so you don't have to implement it yourself. Just add them to your project and go!

* [web_console](https://github.com/richardanaya/web_console)
* [web_random](https://github.com/richardanaya/web_random)
* [web_timer](https://github.com/richardanaya/web_timer)


## How it works

1. Get a handle to some Javascript function using the `js!` macro. Re-use this handle as often as possible.
2. If you are invoking this function as a regular function, use the appropriate `invoke_*` function based on the number of arguments you are passing (`invoke_1`,`invoke_7`,etc.).
3. If you are invoking this function as a method of an object represented by a `JSValue`, use the appropriate `call_*` function based on the number of arguments you are passing (`call_1`,`invoke_7`,etc.) and make sure your object is the first paramter.


# Don't like Rust?

The script `js_ffi.js` has nothing Rust specific.  

* Operations execute through an interface specified in this [`js_ffi.h`](https://github.com/richardanaya/js_ffi/blob/master/js_ffi.h)
* `js_ffi` expects an entry point `main()`
* If you plan on having your module receive data it must implement `jsffimalloc(i32) -> i32`
* If you plan on having your module receive callbacks it must implement `jsfficallback(i32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32)`
* strings are simply c-strings in memory that end in a `0` character.

# Run as a webworker

```js
// main.js
let w = new Worker("worker.js");
w.postMessage("go");
```

```js
// worker.js
importScripts("https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js")
onmessage = function() {
    js_ffi.run("helloworld.wasm");
}
```

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
