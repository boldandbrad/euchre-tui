use crate::engine::game::GameState;
use crate::interface::interface_callback::InterfaceCallback;
use crate::interface::traits::Screen;
use crossterm::event::{KeyCode, KeyEventKind};
use name_maker::RandomNameGenerator;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Margin,
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{block::Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io::Result;
use tui_textarea::TextArea;

#[derive(Debug, Default, PartialOrd, PartialEq)]
pub enum SetupState {
    #[default]
    UserName,
    TeamName,
    Done,
}

impl SetupState {
    pub fn next_step(&self) -> Self {
        match self {
            SetupState::UserName => SetupState::TeamName,
            SetupState::TeamName => SetupState::Done,
            SetupState::Done => SetupState::Done,
        }
    }

    pub fn prev_step(&self) -> Self {
        match self {
            SetupState::Done => SetupState::TeamName,
            SetupState::TeamName => SetupState::UserName,
            SetupState::UserName => SetupState::UserName,
        }
    }
}

// TODO: implement confirming inputs and initializing a new game state / passing data to game_screen
#[derive(Debug, Default)]
pub struct SetupScreen {
    pub state: SetupState,
    pub user_name_textarea: TextArea<'static>,
    pub team_name_textarea: TextArea<'static>,
    pub partner_name: String,
    pub opp1_name: String,
    pub opp2_name: String,
}

impl SetupScreen {
    pub fn new() -> Self {
        let mut user_name_textarea = TextArea::default();
        let text_area_style = Style {
            fg: None,
            bg: None,
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        };
        user_name_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(text_area_style)
                .title("Your name"),
        );

        let mut team_name_textarea = TextArea::default();
        team_name_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(text_area_style)
                .title("Team name"),
        );
        SetupScreen {
            state: SetupState::default(),
            user_name_textarea,
            team_name_textarea,
            partner_name: String::new(),
            opp1_name: String::new(),
            opp2_name: String::new(),
        }
    }

    fn set_state(&mut self, state: SetupState) {
        self.state = state;
    }

    fn submit(&mut self) {
        let rng = RandomNameGenerator::init();
        self.partner_name = rng.generate().first_name;
        self.opp1_name = rng.generate().first_name;
        self.opp2_name = rng.generate().first_name;
    }
}

impl Screen for SetupScreen {
    fn render(&mut self, frame: &mut Frame, _game_state: &GameState) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Fill(3),
            ])
            .split(frame.size());

        // TODO: add a screen header with a description of the screen and how to use it
        // TODO: create layout similar to splash screen with centered text areas and paragraphs

        frame.render_widget(self.user_name_textarea.widget(), layout[0]);
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            match self.state {
                SetupState::UserName => match key_event.code {
                    // TODO: this should be global to all states?
                    // TODO: how to handle moving to previous?
                    KeyCode::Enter => {
                        self.set_state(self.state.next_step());
                    }
                    _ => {
                        // TODO: validate input and handle special keys
                        self.user_name_textarea.input(key_event);
                    }
                },
                _ => {}
            };
        }
        None
    }
}
