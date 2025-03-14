use log::{Level, Metadata, Record};

use crate::{get_offset_time, ConsoleLogger};

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= self.0
  }

  fn log(&self, r: &Record) {
    if !self.enabled(r.metadata()) {
      return;
    }

    let lv = r.level();
    let module = r.module_path().unwrap_or("");
    let now = get_offset_time();
    let offset_str = if now.offset().is_utc() { "Z" } else { "" };

    let org_line = r.line().unwrap_or(0);
    let line = {
      use owo_colors::OwoColorize;
      org_line.blue()
    };

    let colored_lv = {
      use owo_colors::OwoColorize;
      use Level::*;
      let sty = owo_colors::Style::new();
      match lv {
        Error => lv.style(sty.red().bold()),
        Warn => lv.style(sty.yellow().bold()),
        Info => lv.style(sty.green()),
        Debug => lv.style(sty.blue()),
        _ => lv.style(sty.cyan()),
      }
    };

    let fmt_str = format!(
            "{hour:02}:{min:02}:{sec:02}.{ms:03}{offset_str} [{colored_lv}] {module}:{line} {args}",
            hour = now.hour(),
            min = now.minute(),
            sec = now.second(),
            ms = now.millisecond(),
            args = r.args()
        );
    Self::output(lv as _, &fmt_str);
  }

  fn flush(&self) {}
}
