//!
//! ---
//!
//! Due to some browsers not supporting `console.debug()`, there is no `c_dbg!`
//! macro.
//!
//!
//! - [clg!](clg) `console.log()`
//! - [c_err!](c_err) `console.error()`
//! - [c_warn!](c_warn) `console.warn()`
//! - [c_info!](c_info) `console.info()`
//! - [c_trace!](c_trace) `console.trace()`

/// Outputs messages to web console.
///
///  # Example
///
/// ```no_run,rust
/// clg!("{message} {msg_2}")
/// ```
///
/// This is equivalent to calling `console.log()` in js.
///
/// ```js
/// console.log(`${message} ${msg_2}`)
/// ```
#[macro_export]
macro_rules! clg {
    ($($t:tt)*) => ($crate::console::log_1(&format!($($t)*).into()))
}

/// Outputs error messages to web console.
///
///  # Example
///
/// ```no_run,rust
/// c_err!("{error_message} {err_2}")
/// ```
///
///  This is equivalent to calling `console.error()` in js
///
/// ```js
/// console.error(`${error_message} ${err_2}`)
/// ```
#[macro_export]
macro_rules! c_err {
    ($($t:tt)*) => ($crate::console::error_1(&format!($($t)*).into()))
}

/// Outputs warning messages to web console.
///
///  # Example
///
/// ```no_run,rust
/// c_warn!("{warn_message} {warn_2}")
/// ```
///
///  This is equivalent to calling `console.warn()` in js
///
/// ```js
/// console.error(`${warn_message} ${warn_2}`)
/// ```
#[macro_export]
macro_rules! c_warn {
    ($($t:tt)*) => ($crate::console::warn_1(&format!($($t)*).into()))
}

/// Outputs information to web console.
///
///  # Example
///
/// ```no_run,rust
/// c_info!("{info_1} {info_2}")
/// ```
///
///  This is equivalent to calling `console.warn()` in js
///
/// ```js
/// console.error(`${info_1} ${info_2}`)
/// ```
#[macro_export]
macro_rules! c_info {
    ($($t:tt)*) => ($crate::console::info_1(&format!($($t)*).into()))
}

/// Outputs trace messages to web console.
///
///  # Example
///
/// ```no_run,rust
/// c_trace!("{msg_1} {msg_2}")
/// ```
///
///  This is equivalent to calling `console.trace()` in js
///
/// ```js
/// console.error(`${msg_1} ${msg_2}`)
/// ```
#[macro_export]
macro_rules! c_trace {
    ($($t:tt)*) => ($crate::console::trace_1(&format!($($t)*).into()))
}
