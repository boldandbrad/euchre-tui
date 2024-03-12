use crate::engine::game::GameState;
use crate::interface::interface_callback::InterfaceCallback;
use crate::interface::traits::Screen;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::Frame;
use std::io::Result;

pub struct SetupScreen {}

impl SetupScreen {
    pub fn new() -> Self {
        SetupScreen {}
    }
}

impl Screen for SetupScreen {
    fn render(&mut self, _frame: &mut Frame, _game_state: &GameState) -> Result<()> {
        // TODO: implement
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Enter => return Some(InterfaceCallback::PlayGame),
                _ => {}
            }
        }
        None
    }
}
