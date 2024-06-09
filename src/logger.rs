use super::ConsoleLogger;
use crate::{c_err, log_level::LogLevel};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(js_class = _clg_ConsoleLogger)]
impl ConsoleLogger {
    /// Initializes the global logger, when `level` is None, the default value will be used, which is `Info`.
    ///
    /// When Level is `Some(LogLevel::Off)`, initialization will not be performed.
    ///
    /// # Example
    ///
    /// ## Configure in Rust:
    ///
    ///  src/lib.rs
    ///
    /// ```
    /// use clg::log_level::LogLevel;
    /// use clg::ConsoleLogger;
    ///
    /// #[wasm_bindgen]
    /// pub fn init_logger() {
    ///     ConsoleLogger::init(Some(LogLevel::Debug));
    /// }
    /// ```
    ///
    /// js/index.cjs
    ///
    /// ```js
    /// const wasm = require("./pkg/[custom-wasm-glue].js");
    ///
    /// const _init = wasm.init_logger();
    ///
    /// wasm._clg_testLogger();
    /// ```
    ///
    /// > Within the same thread, the logger can only be initialized once. If you choose to initialize in js, do not call `init_logger()` in rust.
    ///
    /// ## Configure in Javascript:
    ///
    ///  src/lib.rs
    ///
    /// ```rust
    /// #[allow(unused_imports)]
    /// use clg::ConsoleLogger as _;
    /// ```
    ///
    /// js/index.cjs
    ///
    /// ```js
    /// const wasm = require("./pkg/[custom-wasm-glue].js");
    ///
    /// const lv = wasm._clg_newLogLevel("info");
    /// const _init = new wasm._clg_ConsoleLogger(lv);
    ///
    /// wasm._clg_testLogger();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn init(level: Option<LogLevel>) -> Self {
        let instance = match level {
            Some(lv @ LogLevel::Off) => return Self(lv.into()),
            Some(lv) => Self(lv.into()),
            _ => Default::default(),
        };

        instance
            .real_init()
            .unwrap_or_else(|e| {
                c_err!("{e:?}");
                panic!()
            });
        instance
    }

    pub(crate) fn real_init(self) -> Result<(), JsValue> {
        let lv = self.0;
        log::set_boxed_logger(Box::new(self))
            .map(|_| log::set_max_level(lv))
            .map_err(|e| e.to_string().into())
    }

    /// Gets the Log Level (String)
    pub fn get_level(&self) -> String {
        self.0.to_string()
    }

    /// Gets the Log Level (usize)
    ///
    /// ```no_run
    /// Off =>0,
    /// Error =>1,
    /// Warn =>2,
    /// Info =>3,
    /// Debug =>4,
    /// Trace =>5
    /// ```
    pub fn get_level_num(&self) -> usize {
        self.0 as _
    }

    /// Calls the specified console function according to log_level.
    ///
    /// ```no_run
    /// error => console.error(msg),
    /// warn => console.warn(msg),
    /// info | debug => console.info(msg),
    /// trace => console.trace(msg)
    /// ```
    ///
    /// Since some browsers don't support `console.debug()`, use `console.info()` when `level == debug`
    ///
    /// ---
    ///
    /// # Example
    ///
    /// ```no_run
    /// let lv = LogLevel::Debug;
    /// output(lv as _, "dbg message")
    /// ```
    ///
    pub fn output(level: usize, content: &str) {
        use crate::console::*;
        let o = content;
        match level {
            1 => err(o),
            2 => warn(o),
            3 | 4 => info(o),
            _ => trace(o),
        }
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        Self(log::LevelFilter::Info)
    }
}

impl core::ops::Deref for ConsoleLogger {
    type Target = log::LevelFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
