use crate::engine::game::GameState;
use ratatui::Frame;
use std::io::Result;

pub trait Screen {
    fn render(&mut self, _frame: &mut Frame, _game_state: &GameState) -> Result<()>;
}
