use crate::interface::{interface_callback::InterfaceCallback, screens::Screen};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Margin,
    style::Stylize,
    text::{Line, Text},
    widgets::{block::Block, Borders, Paragraph},
    Frame,
};
use std::io::Result;

#[allow(dead_code)]
const TITLE_SMALL: [&str; 4] = [
    "              _                _        _",
    "  ___ _  _ __| |_  _ _ ___ ___| |_ _  _(_)",
    " / -_) || / _| ' \\| '_/ -_)___|  _| || | |",
    " \\___|\\_,_\\__|_||_|_| \\___|    \\__|\\_,_|_|",
];
#[allow(dead_code)]
const TITLE_SMALL_BLOCK: [&str; 4] = [
    "             _               _       _ ",
    " ___ _ _ ___| |_ ___ ___ ___| |_ _ _|_|",
    "| -_| | |  _|   |  _| -_|___|  _| | | |",
    "|___|___|___|_|_|_| |___|   |_| |___|_|",
];
#[allow(dead_code)]
const TITLE_SMALL_SLANT: [&str; 4] = [
    "               __               __       _",
    " ___ __ ______/ /  _______ ____/ /___ __(_)",
    "/ -_) // / __/ _ \\/ __/ -_)___/ __/ // / /",
    "\\__/\\_,_/\\__/_//_/_/  \\__/    \\__/\\_,_/_/",
];
#[allow(dead_code)]
const TITLE: [&str; 5] = [
    "                  _                    _         _ ",
    "   ___ _   _  ___| |__  _ __ ___      | |_ _   _(_)",
    "  / _ \\ | | |/ __| '_ \\| '__/ _ \\_____| __| | | | |",
    " |  __/ |_| | (__| | | | | |  __/_____| |_| |_| | |",
    "  \\___|\\__,_|\\___|_| |_|_|  \\___|      \\__|\\__,_|_|",
];
#[allow(dead_code)]
const TITLE_BLOCK: [&str; 5] = [
    "               _                    _         _ ",
    " ___ _   _ ___| |___ ____ ___     _| |_ _   _|_|",
    "| _ | | | |  _|  _  |  __| _ |___|_   _| | | | |",
    "| __| |_| | |_| | | | |  | __|_____| | | |_| | |",
    "|___|_____|___|_| |_|_|  |___|     |_| |_____|_|",
];
#[allow(dead_code)]
const TITLE_SLANT: [&str; 5] = [
    "                   __                    __        _ ",
    "  ___  __  _______/ /_  ________        / /___  __(_)",
    " / _ \\/ / / / ___/ __ \\/ ___/ _ \\______/ __/ / / / / ",
    "/  __/ /_/ / /__/ / / / /  /  __/_____/ /_/ /_/ / /  ",
    "\\___/\\__,_/\\___/_/ /_/_/   \\___/      \\__/\\__,_/_/   ",
];

const VERSION: &str = env!("CARGO_PKG_VERSION");

// splash screen repr
#[derive(Default)]
pub struct SplashScreen {
    title: Paragraph<'static>,
    menu_option_text: Vec<String>,
}

// TODO: add ability to traverse menu with arrow keys and select a button with enter
impl SplashScreen {
    pub fn new() -> Self {
        let title = paragraph_from_multiline_string(TITLE).green();
        let menu_option_text = vec![
            " New Game (n)".to_string(),
            // " Resume Game (l)".to_string(),
            // " Learn Euchre (e)".to_string(),
            // " Settings (s)".to_string(),
            // " Scores (c)".to_string(),
            // " About (a)".to_string(),
            // " Help (h)".to_string(),
            " Exit (Esc/Ctrl-C)".to_string(),
        ];
        Self {
            title,
            menu_option_text,
        }
    }

    fn build_menu_panel(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout_menu_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(42),
                Constraint::Fill(1),
            ])
            .split(area);

        let layout_menu = Layout::default()
            .direction(Direction::Vertical)
            .constraints::<Vec<Constraint>>(
                (0..=self.menu_option_text.len())
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(layout_menu_row[1]);

        for i in 0..self.menu_option_text.len() {
            let menu_option = Paragraph::new(self.menu_option_text[i].clone())
                .block(Block::new().borders(Borders::ALL));

            frame.render_widget(menu_option, layout_menu[i]);
        }
        Ok(())
    }
}

impl Screen for SplashScreen {
    // render the splash screen to the frame
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Fill(3),
            ])
            .split(frame.size());
        // app title
        frame.render_widget(self.title.clone(), layout[1]);
        // app version
        frame.render_widget(
            Paragraph::new(format!("v{VERSION}")).alignment(Alignment::Center),
            layout[2].inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
        );
        // app menu
        self.build_menu_panel(frame, layout[3])?;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            #[allow(clippy::single_match)] // more will be added to this in the future
            match key_event.code {
                KeyCode::Char('n') => return Some(InterfaceCallback::SetupNewGame),
                _ => {}
            }
        }
        None
    }

    fn handle_tick_event(&mut self) -> Option<InterfaceCallback> {
        None
    }
}

fn paragraph_from_multiline_string(multiline_string: [&'static str; 5]) -> Paragraph<'static> {
    let mut lines = Vec::new();
    for line in multiline_string {
        lines.push(Line::from(line));
    }
    Paragraph::new(Text::from(lines)).alignment(Alignment::Center)
}
