use super::interface_callback::InterfaceCallback;
use crate::engine::game::GameState;
use crate::interface::traits::Screen;
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
const TITLE_SMALL: [&'static str; 4] = [
    "              _                _        _",
    "  ___ _  _ __| |_  _ _ ___ ___| |_ _  _(_)",
    " / -_) || / _| ' \\| '_/ -_)___|  _| || | |",
    " \\___|\\_,_\\__|_||_|_| \\___|    \\__|\\_,_|_|",
];
#[allow(dead_code)]
const TITLE_SMALL_BLOCK: [&'static str; 4] = [
    "             _               _       _ ",
    " ___ _ _ ___| |_ ___ ___ ___| |_ _ _|_|",
    "| -_| | |  _|   |  _| -_|___|  _| | | |",
    "|___|___|___|_|_|_| |___|   |_| |___|_|",
];
#[allow(dead_code)]
const TITLE_SMALL_SLANT: [&'static str; 4] = [
    "               __               __       _",
    " ___ __ ______/ /  _______ ____/ /___ __(_)",
    "/ -_) // / __/ _ \\/ __/ -_)___/ __/ // / /",
    "\\__/\\_,_/\\__/_//_/_/  \\__/    \\__/\\_,_/_/",
];
#[allow(dead_code)]
const TITLE: [&'static str; 5] = [
    "                  _                    _         _ ",
    "   ___ _   _  ___| |__  _ __ ___      | |_ _   _(_)",
    "  / _ \\ | | |/ __| '_ \\| '__/ _ \\_____| __| | | | |",
    " |  __/ |_| | (__| | | | | |  __/_____| |_| |_| | |",
    "  \\___|\\__,_|\\___|_| |_|_|  \\___|      \\__|\\__,_|_|",
];
#[allow(dead_code)]
const TITLE_BLOCK: [&'static str; 5] = [
    "               _                    _         _ ",
    " ___ _   _ ___| |___ ____ ___     _| |_ _   _|_|",
    "| _ | | | |  _|  _  |  __| _ |___|_   _| | | | |",
    "| __| |_| | |_| | | | |  | __|_____| | | |_| | |",
    "|___|_____|___|_| |_|_|  |___|     |_| |_____|_|",
];
#[allow(dead_code)]
const TITLE_SLANT: [&'static str; 5] = [
    "                   __                    __        _ ",
    "  ___  __  _______/ /_  ________        / /___  __(_)",
    " / _ \\/ / / / ___/ __ \\/ ___/ _ \\______/ __/ / / / / ",
    "/  __/ /_/ / /__/ / / / /  /  __/_____/ /_/ /_/ / /  ",
    "\\___/\\__,_/\\___/_/ /_/_/   \\___/      \\__/\\__,_/_/   ",
];

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SplashScreen {
    title: Paragraph<'static>,
    menu_option_text: Vec<String>,
}

// TODO: add ability to traverse menu with arrow keys and select a button with enter
impl SplashScreen {
    pub fn new() -> Self {
        let title = paragraph_from_multiline_string(TITLE).green();
        let mut menu_option_text = vec![];
        menu_option_text.push(" New Game (n)".to_string());
        // menu_option_text.push(" Resume Game (l)".to_string());
        // menu_option_text.push(" Learn Euchre (e)".to_string());
        // menu_option_text.push(" Settings (s)".to_string());
        // menu_option_text.push(" Scores (c)".to_string());
        // menu_option_text.push(" About (a)".to_string());
        // menu_option_text.push(" Help (h)".to_string());
        menu_option_text.push(" Exit (Esc/Ctrl-C)".to_string());
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
    fn render(&mut self, frame: &mut Frame, _game_state: &GameState) -> Result<()> {
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
            match key_event.code {
                KeyCode::Char('n') => return Some(InterfaceCallback::SetupNewGame),
                _ => {}
            }
        }
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
