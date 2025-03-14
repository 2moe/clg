//! A logger specifically designed for wasm32-unknown-unknown.
//!
//! Go to the [git repository](https://github.com/2moe/clg) for more information.
//

#[cfg(not(all(
  any(target_arch = "wasm32", target_arch = "wasm64"),
  target_os = "unknown"
)))]
pub const SUPPORTED: bool = false;

#[cfg(all(
  any(target_arch = "wasm32", target_arch = "wasm64"),
  target_os = "unknown"
))]
pub const SUPPORTED: bool = true;

/// web console
pub mod console;

#[cfg(feature = "logger")]
mod log_impl;

#[cfg(feature = "logger")]
/// Contains the `LogLevel` enum and its methods.
pub mod log_level;

/// Allows macros to be called separately to output to the web console, even if
/// the logger is not initialized.
pub mod macros;

#[cfg(feature = "logger")]
type Any = wasm_bindgen::JsValue;

#[cfg(feature = "logger")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
/// A logger used to output logs to the web console.
#[wasm_bindgen(js_name = _clgConsoleLogger)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ConsoleLogger(log::LevelFilter);

/// Gets the time of the current timezone offset. If the acquisition fails,
/// fallback to UTC time.
#[cfg(feature = "logger")]
pub fn get_offset_time() -> time::OffsetDateTime {
  use time::OffsetDateTime as offset;
  offset::now_local().unwrap_or_else(|_| offset::now_utc())
}

#[cfg(feature = "logger")]
// #[cfg(test)]
mod test_wasm {
  use wasm_bindgen::prelude::wasm_bindgen;

  #[wasm_bindgen(js_name = __clgTestLogger)]
  pub fn test_logger() {
    use log::*;
    trace!("Trace");
    debug!("DBG");
    info!("information");
    warn!("warning");
    error!("panic");
  }
}
