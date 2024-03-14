use crate::engine::game::GameState;
use crate::interface::{
    game_screen::GameScreen, interface_callback::InterfaceCallback, setup_screen::SetupScreen,
    splash_screen::SplashScreen, traits::Screen,
};
use ratatui::Frame;
use std::io::Result;

#[derive(Default)]
pub enum InterfaceState {
    #[default]
    Splash,
    GameSetup,
    GameTable,
}

pub struct Interface {
    state: InterfaceState,
    pub splash_screen: SplashScreen,
    pub setup_screen: SetupScreen,
    pub game_screen: GameScreen,
}

impl Interface {
    pub fn new() -> Self {
        let splash_screen = SplashScreen::new();
        let setup_screen = SetupScreen::new();
        let game_screen = GameScreen::new();
        Self {
            state: InterfaceState::default(),
            splash_screen,
            setup_screen,
            game_screen,
        }
    }

    pub fn set_state(&mut self, state: InterfaceState) {
        self.state = state;
    }

    pub fn render(&mut self, frame: &mut Frame, game_state: &GameState) -> Result<()> {
        self.get_active_screen_mut().render(frame, game_state)?;
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        let callback = self.get_active_screen_mut().handle_key_event(key_event);
        match callback {
            Some(InterfaceCallback::StartGame) => self.set_state(InterfaceState::GameTable),
            Some(InterfaceCallback::SetupNewGame) => self.set_state(InterfaceState::GameSetup),
            Some(InterfaceCallback::QuitToSplash) => self.set_state(InterfaceState::Splash),
            _ => return callback,
        }
        None
    }

    fn get_active_screen_mut(&mut self) -> &mut dyn Screen {
        match self.state {
            InterfaceState::Splash => &mut self.splash_screen,
            InterfaceState::GameSetup => &mut self.setup_screen,
            InterfaceState::GameTable => &mut self.game_screen,
        }
    }
}
