use euchre_tui::app::App;
use std::io::Result;

fn main() -> Result<()> {
    App::new().run()?;
    Ok(())
}
