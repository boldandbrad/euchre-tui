use crate::engine::game::GameState;
use crate::interface::game_screen::GameScreen;
use crate::interface::traits::Screen;
use ratatui::Frame;
use std::io::Result;

#[derive(Default)]
pub enum InterfaceState {
    // TODO: make splash default in the future
    Splash,
    Menu,
    NewGame,
    #[default]
    GameTable,
}

pub struct Interface {
    state: InterfaceState,
    pub game_screen: GameScreen,
}

impl Interface {
    pub fn new() -> Self {
        let game_screen = GameScreen::new();
        Self {
            state: InterfaceState::default(),
            game_screen,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, game_state: &GameState) -> Result<()> {
        match self.state {
            InterfaceState::Splash => {}
            InterfaceState::Menu => {}
            InterfaceState::NewGame => {}
            InterfaceState::GameTable => {
                self.game_screen.render(frame, game_state)?;
            }
        }
        Ok(())
    }
}
