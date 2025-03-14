# clg

专为 wasm32-unknown-unknown 打造的 logger (日志记录器)。

[![crates.io](<https://img.shields.io/crates/v/clg.svg?label=lib(clg)>)](https://crates.io/crates/clg)

[![Documentation](https://img.shields.io/docsrs/clg?label=docs.rs)](https://docs.rs/clg)    [![Apache-2 licensed](https://img.shields.io/crates/l/clg)](../License)

| Language/語言          | ID         |
| ---------------------- | ---------- |
| 中文                   | zh-Hans-CN |
| [English](./Readme.md) | en-Latn-US |

## Q&A

### Q: 这玩意儿有什么用？

A: 用 rust 将日志输出到 Web/Node.js/Deno 的控制台。

rust:

```rust
  #[wasm_bindgen(js_name = __clgTestLogger)]
  pub fn test_logger() {
    use log::*;
    trace!("Trace");
    debug!("DBG");
    info!("information");
    warn!("warning");
    error!("panic");
  }
```

![rust-code](https://github.com/2moe/clg/assets/25324935/49c23c65-e9de-4cb0-aa57-7a3e51076778)

js:

```js
const wasm = require("/path/to/your_wasm_glue.js")

const logLevel = wasm._clgNewLogLevel("debug")

const initLogger = new wasm._clgConsoleLogger(logLevel)

wasm.__clgTestLogger()
```

![js-glue](https://github.com/2moe/clg/assets/25324935/7873a1cc-9764-48b6-861d-b8f9d03693d0)

### Q: 为什么不直接用标准库？

A: 对于 **wasm32-wasi**，我们可以直接使用 rust std 的 `eprintln`，以及常规的 logger。

然而，对于 **wasm32-unknown-unknown**，我们得要调用 web 控制台，也就是调用 js 的 `console.log()` 和 `console.error()` 等函数。

### Q: 要怎么用？

A:

- 如果已经配置了 logger, 那就用 log 宏：
  - 例如 `warn!("{warn_1} {warn_2}")`、 `debug!("{dbg_1} {dbg_2}")`
  - 调用 `log::debug!("这是一条调试消息")` 后，输出的内容类似于 `06:23:16:223 [DEBUG] clg::test_wasm:186 这是一条调试消息`

> log 宏:
>
> - `log::error!()`
> - `log::warn!()`
> - `log::info!()`
> - `log::debug!()`
> - `log::trace!()`

- 如果没有配置 logger, 那就用 clg 宏：
  - 例如 `clg!("{message_1}")`
  - 调用 `clg` 宏时，不会包含额外的日志信息，也不会自动判断 log-level (日志输出级别) 。
  - 由于部分浏览器不支持 `console.debug()` ，因此没有 `c_dbg!()`

在 rust 中调用 `c_err!("{msg1} {msg2}")`, 相当于在 js 中调用:

```js
console.error(`${msg1} ${msg2}`)
```

> clg 宏：
>
> - `clg::clg!()` => `console.log()`
> - `clg::c_err!()` => `console.error()`
> - `clg::c_warn!()` => `console.warn()`
> - `clg::c_info!()` => `console.info()`
> - `clg::c_trace!()` =>  `console.trace()`

更多使用说明，详见下文。

## Usage

### Step1. 安装 wasm-pack

```sh
cargo install wasm-pack
```

### Step2. 添加依赖

**Cargo.toml**:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.92"
log = { version = "0.4.21", features = ["std"] }
clg = "0.0.2"
# js-sys = "0.3.69"
# web-sys = "0.3.69"
```

### Step3. 配置 Logger

先来介绍一下什么是 log level。

从低到高分别为

- Off    (无 Log)
- Error  (错误)
- Warn   (警告)
- Info   (信息)
- Debug  (调试)
- Trace  (追踪)

级别越高，打印的内容越详细。

低级别的 level 无法显示高级别 level 的消息。

假设当前 level 为 warn，且存在如下代码：

```rust
use log::{debug, error, warn};
let id = "div_2";
debug!("正在获取 DOM 元素, id: {id}");
let Some(element) = document.get_element_by_id(id) else {
    warn!("无法获取元素");
    error!("出错惹");
    return;
};
```

假设 debug = 3, warn = 2, error = 1

由于 debug(3) > warn(2) > error(1)

当 level = warn(2) 时，只能显示 warn(2) 和 error(1)，不会显示 debug(3) 日志。

也就是说，上面的代码不会输出 "正在获取 DOM 元素..."。

#### 在 rust 中配置

您既可以在 rust 中配置 Log Level，亦可在 js 中配置。

**src/lib.rs**:

```rust
use clg::{log_level::LogLevel, ConsoleLogger};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = initLogger)]
pub fn init_logger() {
    ConsoleLogger::init(Some(LogLevel::Debug));
}
```

编译与打包:

```sh
# 您可以将 nodejs 改成 deno，最后用 deno 来执行
wasm-pack build --release --target nodejs --out-dir pkg --out-name wasm
```

**js/index.cjs**:

```js
// index.mjs: import * as wasm from "../pkg/wasm.js";
const wasm = require("../pkg/wasm.js");

const _init = wasm.initLogger();
wasm.__clgTestLogger();
```

最后，使用 nodejs 运行 index.cjs:

```sh
node js/index.cjs
```

#### 在 js 中配置

> 注：logger 是全局的。每个线程只能运行一个 logger，不要多次初始化 logger。
> 一旦 logger 初始化完成，在其他地方 (rust 的函数内) 就能直接用 `log` 宏了。

**src/lib.rs**:

```rust
#[allow(unused_imports)]
use clg::ConsoleLogger as _;
```

**js/index.cjs**:

```js
const wasm = require("../pkg/wasm.js");

// const lv = wasm._clgLogLevel.Warn;
const lv = wasm._clgNewLogLevel("debug");
const _init = new wasm._clgConsoleLogger(lv);

wasm.__clgTestLogger();
```

运行:

```sh
node js/index.cjs
```
