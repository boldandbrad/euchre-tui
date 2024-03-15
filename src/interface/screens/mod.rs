pub mod game_screen;
pub mod setup_screen;
pub mod splash_screen;

use crate::interface::interface_callback::InterfaceCallback;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;

pub trait Screen {
    fn render(&mut self, _frame: &mut Frame) -> Result<()>;
    fn handle_key_event(&mut self, _key_event: KeyEvent) -> Option<InterfaceCallback>;
    fn handle_tick_event(&mut self) -> Option<InterfaceCallback>;
}
