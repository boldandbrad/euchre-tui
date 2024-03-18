pub mod components;
pub mod interface_callback;
pub mod layouts;
pub mod screens;

use crate::interface::{
    interface_callback::InterfaceCallback,
    screens::{
        game_screen::GameScreen, setup_screen::SetupScreen, splash_screen::SplashScreen, Screen,
    },
};
use ratatui::Frame;
use std::io::Result;

// interface state repr
#[derive(Default)]
pub enum InterfaceState {
    #[default]
    Splash,
    GameSetup,
    GameTable,
}

// interface repr
#[derive(Default)]
pub struct Interface {
    state: InterfaceState,
    pub splash_screen: SplashScreen,
    pub setup_screen: SetupScreen,
    pub game_screen: GameScreen,
}

impl Interface {
    // create a new interface instance
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

    // set the interface state
    pub fn set_state(&mut self, state: InterfaceState) {
        self.state = state;
    }

    // render the interface to a frame
    pub fn render(&mut self, frame: &mut Frame) -> Result<()> {
        self.get_active_screen_mut().render(frame)?;
        Ok(())
    }

    // handle key events based on interface state
    pub fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        let callback = self.get_active_screen_mut().handle_key_event(key_event);
        match callback {
            Some(InterfaceCallback::StartGame { game }) => {
                self.set_state(InterfaceState::GameTable);
                self.game_screen.set_game(*game);
            }
            Some(InterfaceCallback::SetupNewGame) => self.set_state(InterfaceState::GameSetup),
            Some(InterfaceCallback::QuitToSplash) => self.set_state(InterfaceState::Splash),
            _ => return callback,
        }
        None
    }

    // handle tick events based on interface state
    pub fn handle_tick_event(&mut self) -> Option<InterfaceCallback> {
        let _callback = self.get_active_screen_mut().handle_tick_event();
        None
    }

    // get mutable reference to the active screen
    fn get_active_screen_mut(&mut self) -> &mut dyn Screen {
        match self.state {
            InterfaceState::Splash => &mut self.splash_screen,
            InterfaceState::GameSetup => &mut self.setup_screen,
            InterfaceState::GameTable => &mut self.game_screen,
        }
    }
}
