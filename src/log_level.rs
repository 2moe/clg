use log::LevelFilter;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::c_err;

#[wasm_bindgen(js_name = _clg_LogLevel)]
/// LogLevel and `log::LevelFilter` can be converted to each other.
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Creates a new [LogLevel](crate::log_level::LogLevel) enum instance from `&str`.
///
/// `s` is case-insensitive, optional values: off, error, warn, info, debug, trace.
///
/// > The reason for not creating a `LogLevel::from_str()` method is to be compatible with wasm_bindgen.
///
/// rust:
///
/// ```no_run
/// let level = new_log_level("info")
/// let level = new_log_level("debug")
/// ```
///
/// Or you can also create it in javascript:
///
/// ```js
/// const wasm = require("./pkg/[custom-wasm-glue].js");
///
/// // const level = wasm._clg_LogLevel.Warn;
/// const level = wasm._clg_newLogLevel("warn");
/// ```
#[wasm_bindgen(js_name = _clg_newLogLevel)]
pub fn new_log_level(s: &str) -> LogLevel {
    use core::str::FromStr;
    LevelFilter::from_str(s)
        .unwrap_or_else(|e| {
            const MSG: &str = "Failed to create ConsoleLogger";
            c_err!(
                "[ERROR] {module}:{line} {e}\nPanic: {MSG}",
                module = module_path!(),
                line = line!()
            );
            panic!("{MSG}")
        })
        .into()
}

impl From<LogLevel> for LevelFilter {
    fn from(val: LogLevel) -> Self {
        use LogLevel::*;
        match val {
            Off => Self::Off,
            Error => Self::Error,
            Warn => Self::Warn,
            Info => Self::Info,
            Debug => Self::Debug,
            Trace => Self::Trace,
        }
    }
}

impl From<LevelFilter> for LogLevel {
    fn from(val: LevelFilter) -> Self {
        use LevelFilter::*;
        match val {
            Off => Self::Off,
            Error => Self::Error,
            Warn => Self::Warn,
            Info => Self::Info,
            Debug => Self::Debug,
            Trace => Self::Trace,
        }
    }
}
