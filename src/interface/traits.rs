use crate::interface::interface_callback::InterfaceCallback;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;

pub trait Screen {
    fn render(&mut self, _frame: &mut Frame) -> Result<()>;
    fn handle_key_event(&mut self, _key_event: KeyEvent) -> Option<InterfaceCallback>;
}
