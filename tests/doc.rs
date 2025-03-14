use std::io;

use testutils::os_cmd::{presets::CargoDoc, RunnableCommand};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default().run()
}
