use crate::app::App;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::panic;

// tui repr
pub struct Tui {
    // terminal interface
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    // create a new tui instance
    pub fn new(terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Self {
        Self { terminal }
    }

    // initialize the tui
    pub fn init(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        // custom panic hook to reset terminal props
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    // draw the tui
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.clear()?;
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    // reset the tui
    fn reset() -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    // exit the tui
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
