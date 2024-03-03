use crate::engine::game::GameState;
use crate::interface::game_screen::GameScreen;
use crate::interface::splash_screen::SplashScreen;
use crate::interface::traits::Screen;
use ratatui::Frame;
use std::io::Result;

#[derive(Default)]
pub enum InterfaceState {
    #[default]
    Splash,
    Menu,
    NewGame,
    GameTable,
}

pub struct Interface {
    state: InterfaceState,
    pub splash_screen: SplashScreen,
    pub game_screen: GameScreen,
}

impl Interface {
    pub fn new() -> Self {
        let splash_screen = SplashScreen::new();
        let game_screen = GameScreen::new();
        Self {
            state: InterfaceState::default(),
            splash_screen,
            game_screen,
        }
    }

    pub fn set_state(&mut self, state: InterfaceState) {
        self.state = state;
    }

    pub fn render(&mut self, frame: &mut Frame, game_state: &GameState) -> Result<()> {
        match self.state {
            InterfaceState::Splash => {
                self.splash_screen.render(frame, game_state)?;
            }
            InterfaceState::Menu => {}
            InterfaceState::NewGame => {}
            InterfaceState::GameTable => {
                self.game_screen.render(frame, game_state)?;
            }
        }
        Ok(())
    }

    pub fn handle_key_event() -> Result<()> {
        Ok(())
    }
}
