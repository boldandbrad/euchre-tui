use clap::Parser;
use euchre_tui::app::App;
use euchre_tui::cli::Args;
use std::io::Result;

fn main() -> Result<()> {
    let _ = Args::parse();
    App::new().run()?;
    Ok(())
}
