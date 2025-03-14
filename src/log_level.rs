use log::LevelFilter;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::c_warn;

#[wasm_bindgen(js_name = _clgLogLevel)]
/// LogLevel and `log::LevelFilter` can be converted to each other.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LogLevel {
  Off,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

/// Creates a new [LogLevel](crate::log_level::LogLevel) enum instance from
/// `&str`.
///
/// `s` is case-insensitive, optional values: off, error, warn, info, debug,
/// trace.
///
/// > The reason for not creating a `LogLevel::from_str()` method is to be
/// > compatible with wasm_bindgen.
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
/// // const level = wasm._clgLogLevel.Warn;
/// const level = wasm._clgNewLogLevel("warn");
/// ```
#[wasm_bindgen(js_name = _clgNewLogLevel)]
pub fn new_log_level(s: &str) -> Option<LogLevel> {
  use core::str::FromStr;

  LevelFilter::from_str(s)
    .inspect_err(|e| {
      const MSG: &str = "Failed to parse &str to LevelFilter";
      c_warn!(
        "[WARN] {module}:{line} {e}\n  {MSG}",
        module = module_path!(),
        line = line!()
      );
    })
    .ok()
    .map(|x| x.into())
}

impl From<&str> for LogLevel {
  fn from(val: &str) -> Self {
    match val {
      "Off" => Self::Off,
      "Error" => Self::Error,
      "Warn" => Self::Warn,
      "Info" => Self::Info,
      "Debug" => Self::Debug,
      "Trace" => Self::Trace,
      _ => Self::Info,
    }
  }
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
