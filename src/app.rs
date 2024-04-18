use crate::interface::{interface_callback::InterfaceCallback, Interface};
use crate::tui::Tui;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    // terminal::size,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    io::{stdout, Result},
    time::{Duration, Instant},
};

// application repr
#[derive(Default)]
pub struct App {
    is_running: bool,
    interface: Interface,
    tick_rate: Duration,
}

impl App {
    // create a new application instance
    pub fn new() -> Self {
        let interface = Interface::new();
        let tick_rate = Duration::from_millis(250);
        App {
            is_running: true,
            interface,
            tick_rate,
        }
    }

    // run the application
    pub fn run(&mut self) -> Result<()> {
        // initialize tui
        let terminal: Terminal<CrosstermBackend<std::io::Stdout>> =
            Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut tui = Tui::new(terminal);
        tui.init()?;

        // TODO: check and handle terminal size
        // let (cols, rows) = size()?;
        // println!("{}x{}", cols, rows);
        // if cols < 120 || rows < 36 {
        //     self.is_running = false;
        // }

        // initialize tick tracker
        let mut last_tick = Instant::now();

        // main application loop
        while self.is_running {
            // draw tui
            tui.draw(self)?;

            // handle key events
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

            // handle tick events
            if last_tick.elapsed() >= self.tick_rate {
                self.interface.handle_tick_event();
                last_tick = Instant::now();
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
        self.is_running = false;
        Ok(())
    }
}
