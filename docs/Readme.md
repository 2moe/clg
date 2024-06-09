# clg

A logger specifically designed for wasm32-unknown-unknown.

[![crates.io](<https://img.shields.io/crates/v/clg.svg?label=lib(clg)>)](https://crates.io/crates/clg)

[![Documentation](https://img.shields.io/docsrs/clg?label=docs.rs)](https://docs.rs/clg)    [![Apache-2 licensed](https://img.shields.io/crates/l/clg.svg)](../License)

| Languages/語言         | ID         |
| ---------------------- | ---------- |
| English                | en-Latn-US |
| [中文](./Readme-zh.md) | zh-Hans-CN |

## Q&A

### Q: What is this used for?

A: Output logs to the Web/Node.js/Deno console in Rust.

### Q: Why not use the standard library directly?

A: For **wasm32-wasi**, we can directly use Rust std's `eprintln`, as well as the regular logger.

However, for **wasm32-unknown-unknown**, we need to call the web console, that is, call js's `console.log()` and `console.error()` functions.

### Q: How to use it?

A:

- If a logger has already been configured, use the log macro:
  - For example, `warn!("{warn_1} {warn_2}")`, `debug!("{dbg_1} {dbg_2}")`
  - After calling `log::debug!("This is a debug message")`, the output content is similar to `06:23:16:223 [DEBUG] clg::test_wasm:186 This is a debug message`

> log macro:
>
> - `log::error!()`
> - `log::warn!()`
> - `log::info!()`
> - `log::debug!()`
> - `log::trace!()`

- If a logger is not configured, use the `clg` macro:
  - For example, `clg!("{message_1}")`
  - When calling the `clg` macro, it will not include additional log information, nor will it automatically determine the log-level.
  - Since some browsers do not support `console.debug()`, there is no `c_dbg!()`

Calling `c_err!("{msg1} {msg2}")` in Rust is equivalent to calling the following in JavaScript:

```js
console.error(`${msg1} ${msg2}`)
```

> clg macro：
>
> - `clg::clg!()` => `console.log()`
> - `clg::c_err!()` => `console.error()`
> - `clg::c_warn!()` => `console.warn()`
> - `clg::c_info!()` => `console.info()`
> - `clg::c_trace!()` =>  `console.trace()`

For more usage instructions, see below.

## Usage

### Step1. Install wasm-pack

```sh
cargo install cargo-binstall
cargo binstall wasm-pack
```

### Step2. Add Deps

**Cargo.toml**:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.92"
log = { version = "0.4.21", features = ["std"] }
clg = "0.0.0"
# js-sys = "0.3.69"
# web-sys = "0.3.69"
```

### Step3. Configure Logger

Let's first introduce what a log level is.

From low to high, they are:

- Off    (No Log)
- Error
- Warn   (Warning)
- Info   (Information)
- Debug
- Trace

The higher the level, the more detailed the printed content.

A lower level cannot display messages of a higher level.

Assume the current level is warn, and the following code exists:

```rust
use log::{debug, error, warn};
let id = "div_2";
debug!("Getting DOM element, id: {id}");
let Some(element) = document.get_element_by_id(id) else {
    warn!("Cannot get element");
    error!("An error occurred");
    return;
};
```

Assume debug = 3, warn = 2, error = 1

Since debug(3) > warn(2) > error(1)

When level = warn(2), only warn(2) and error(1) can be displayed, debug(3) logs will not be displayed.

That is to say, the above code will not output "Getting DOM element...".

#### Configuring in Rust

You can configure the Log Level in Rust, as well as in js.

**src/lib.rs**:

```rust
use clg::{log_level::LogLevel, ConsoleLogger};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = initLogger)]
pub fn init_logger() {
    ConsoleLogger::init(Some(LogLevel::Debug));
}
```

Compile and package:

```sh
# You can change nodejs to deno, and finally use deno to execute wasm.js
wasm-pack build --release --target nodejs --out-dir pkg --out-name wasm
```

**js/index.cjs**:

```js
// index.mjs: import * as wasm from "../pkg/wasm.js";
const wasm = require("../pkg/wasm.js");

const _init = wasm.initLogger();
wasm._clg_testLogger();
```

Finally, run index.cjs with nodejs:

```sh
node js/index.cjs
```

#### Configuring in JS

> Note: The logger is global. Each thread can only run one logger, do not initialize the logger multiple times.
> Once the logger is initialized, you can directly use the `log` macro elsewhere (inside Rust functions).

**src/lib.rs**:

```rust
#[allow(unused_imports)]
use clg::ConsoleLogger as _;
```

**js/index.cjs**:

```js
const wasm = require("../pkg/wasm.js");

// const lv = wasm._clg_LogLevel.Warn;
const lv = wasm._clg_newLogLevel("debug");
const _init = new wasm._clg_ConsoleLogger(lv);

wasm._clg_testLogger();
```

Run:

```sh
node js/index.cjs
```
