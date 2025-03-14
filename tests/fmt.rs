use std::io;

use testutils::os_cmd::{presets::CargoFmt, RunnableCommand};

#[ignore]
#[test]
fn nightly_fmt() -> io::Result<()> {
  CargoFmt::default().run()
}
