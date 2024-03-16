pub mod game_screen;
pub mod setup_screen;
pub mod splash_screen;

use crate::interface::interface_callback::InterfaceCallback;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::{
    layout::Alignment,
    text::{Line, Text},
    widgets::Paragraph,
};
use std::io::Result;

pub trait Screen {
    fn render(&mut self, _frame: &mut Frame) -> Result<()>;
    fn handle_key_event(&mut self, _key_event: KeyEvent) -> Option<InterfaceCallback>;
    fn handle_tick_event(&mut self) -> Option<InterfaceCallback>;
}

// TODO: move this somewhere else
fn paragraph_from_multiline_string(multiline_string: [&'static str; 5]) -> Paragraph<'static> {
    let mut lines = Vec::new();
    for line in multiline_string {
        lines.push(Line::from(line));
    }
    Paragraph::new(Text::from(lines)).alignment(Alignment::Center)
}
