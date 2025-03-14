#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

pub use web_sys::console::{debug_1, error_1, info_1, log_1, trace_1, warn_1};

#[cfg(feature = "logger")]
use crate::Any;

#[cfg(feature = "logger")]
pub(crate) fn err<S: Into<Any>>(s: S) {
  error_1(&s.into())
}
#[cfg(feature = "logger")]
pub(crate) fn info<S: Into<Any>>(s: S) {
  info_1(&s.into())
}
#[cfg(feature = "logger")]
pub(crate) fn warn<S: Into<Any>>(s: S) {
  warn_1(&s.into())
}

// #[cfg(feature = "logger")]
// pub(crate) fn dbg<S: Into<Any>>(s: S) {
//     debug_1(&s.into())
// }

#[cfg(feature = "logger")]
pub(crate) fn trace<S: Into<Any>>(s: S) {
  trace_1(&s.into())
}
