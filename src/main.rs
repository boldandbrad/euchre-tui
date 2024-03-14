use clap::Parser;
use euchre_tui::app::App;
use euchre_tui::cli::Args;
use std::io::Result;

fn main() -> Result<()> {
    // parse cli args
    let _ = Args::parse();

    // create and run the application
    App::new().run()?;
    Ok(())
}
