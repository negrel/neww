use std::fs;

use lib::parse_neww_ui;

use anyhow::Context;

pub fn main() -> Result<(), anyhow::Error> {
    let files = &std::env::args().collect::<Vec<_>>();
    let fpath = files.get(1).context("UI file path is missing")?;

    let file_content = fs::read_to_string(fpath).context("failed to read file")?;
    let gtk_ui = parse_neww_ui(&file_content).context("failed to convert neww UI to GTK UI")?;

    let gtk_ui = quick_xml::se::to_string(&gtk_ui.unwrap())?;
    println!("{gtk_ui}");

    Ok(())
}
