use std::fs;

use lib::{parse_neww, Neww};

use anyhow::Context;

pub fn main() -> Result<(), anyhow::Error> {
    let files = &std::env::args().collect::<Vec<_>>();
    let fpath = files.get(1).context("UI file path is missing")?;

    let file_content = fs::read_to_string(fpath).context("failed to read file")?;
    let neww: Neww = parse_neww(&file_content).context("failed to parse neww file")?;

    let gtk_ui = quick_xml::se::to_string(&neww.interface.windows.first())?;
    println!("{gtk_ui}");

    Ok(())
}
