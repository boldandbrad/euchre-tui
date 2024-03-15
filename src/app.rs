use crate::interface::{interface_callback::InterfaceCallback, Interface};
use crate::tui::Tui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    io::{stdout, Result},
    time::Duration,
};

// application repr
#[derive(Default)]
pub struct App {
    running: bool,
    interface: Interface,
}

impl App {
    // create a new application instance
    pub fn new() -> Self {
        let interface = Interface::new();
        App {
            running: true,
            interface,
        }
    }

    // run the application
    pub fn run(&mut self) -> Result<()> {
        // initialize tui
        let terminal: Terminal<CrosstermBackend<std::io::Stdout>> =
            Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut tui = Tui::new(terminal);
        tui.init()?;

        // main application loop
        while self.running {
            // draw tui
            tui.draw(self)?;

            // handle events
            if crossterm::event::poll(Duration::from_millis(16))? {
                if let Event::Key(key_event) = crossterm::event::read()? {
                    match key_event.code {
                        // exit app on `Esc` or `Ctrl-C`
                        KeyCode::Esc => self.exit()?,
                        KeyCode::Char('c') | KeyCode::Char('C')
                            if key_event.modifiers == KeyModifiers::CONTROL =>
                        {
                            self.exit()?;
                        }
                        _ => {
                            self.handle_key_event(key_event)?;
                        }
                    }
                }
            }
        }

        // exit tui
        tui.exit()?;
        Ok(())
    }

    // handle application key events
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let callback = self.interface.handle_key_event(key_event);
        match callback {
            Some(InterfaceCallback::Exit) => self.exit(),
            _ => Ok(()),
        }
    }

    // render the application's interface
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let _ = self.interface.render(frame);
    }

    // exit the application
    pub fn exit(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }
}
