use crate::engine::game::GameState;
use crate::interface::interface_callback::InterfaceCallback;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;

// TODO: find a way to not need to pass game state into render, only GameScreen needs it?
pub trait Screen {
    fn render(&mut self, _frame: &mut Frame, _game_state: &GameState) -> Result<()>;
    fn handle_key_event(&mut self, _key_event: KeyEvent) -> Option<InterfaceCallback>;
}
