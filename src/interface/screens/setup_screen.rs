use crate::engine::game::Game;
use crate::interface::{
    components::{ascii_text::SETUP_HEADER, inputs::InputStyle, layouts::MenuLayout},
    interface_callback::InterfaceCallback,
    screens::{paragraph_from_multiline_string, Screen},
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use name_maker::RandomNameGenerator;
use ratatui::style::Color;
use ratatui::{
    style::{Modifier, Style},
    widgets::{block::Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io::Result;
use tui_textarea::TextArea;

const MIN_NAME_LENGTH: usize = 3;
const MAX_NAME_LENGTH: usize = 12;

// setup screen state repr
#[derive(Debug, Default, PartialOrd, PartialEq)]
pub enum SetupScreenState {
    #[default]
    UserName,
    TeamName,
    Confirm,
}

impl SetupScreenState {
    pub fn next(&self) -> Self {
        match self {
            SetupScreenState::UserName => SetupScreenState::TeamName,
            SetupScreenState::TeamName => SetupScreenState::Confirm,
            SetupScreenState::Confirm => SetupScreenState::Confirm,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            SetupScreenState::Confirm => SetupScreenState::TeamName,
            SetupScreenState::TeamName => SetupScreenState::UserName,
            SetupScreenState::UserName => SetupScreenState::UserName,
        }
    }
}

// setup screen repr
#[derive(Debug, Default)]
pub struct SetupScreen {
    pub state: SetupScreenState,
    pub user_name_textarea: TextArea<'static>,
    pub team_name_textarea: TextArea<'static>,
}

impl SetupScreen {
    pub fn new() -> Self {
        // initialize text areas
        let mut user_name_textarea = TextArea::default();
        user_name_textarea.set_block(build_textarea_block("User Name".to_string()));
        user_name_textarea.set_cursor_line_style(Style::default());

        let mut team_name_textarea = TextArea::default();
        team_name_textarea.set_block(build_textarea_block("Team Name".to_string()));
        team_name_textarea.set_cursor_line_style(Style::default());

        SetupScreen {
            state: SetupScreenState::default(),
            user_name_textarea,
            team_name_textarea,
        }
    }

    fn set_state(&mut self, state: SetupScreenState) {
        self.state = state;
    }

    fn get_active_textarea_mut(&mut self) -> &mut TextArea<'static> {
        match self.state {
            SetupScreenState::UserName => &mut self.user_name_textarea,
            SetupScreenState::TeamName => &mut self.team_name_textarea,
            SetupScreenState::Confirm => unreachable!(),
        }
    }

    fn init_game(&mut self) -> Game {
        Game::new(
            self.user_name_textarea.lines()[0].trim().to_string(),
            generate_player_name(),
            generate_player_name(),
            generate_player_name(),
            self.team_name_textarea.lines()[0].trim().to_string(),
            "Bad Guys".to_string(),
        )
    }
}

impl Screen for SetupScreen {
    // render the setup screen to the frame
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let menu_layout = MenuLayout::new(frame, 3);

        // screen title
        frame.render_widget(
            paragraph_from_multiline_string(SETUP_HEADER).centered(),
            menu_layout.header_area,
        );

        // screen sub title
        // TODO: add a description of the screen and how to use it to the sub header

        // game setup form
        match self.state {
            SetupScreenState::UserName => {
                activate_textarea(&mut self.user_name_textarea);
                deactivate_textarea(&mut self.team_name_textarea);
            }
            SetupScreenState::TeamName => {
                deactivate_textarea(&mut self.user_name_textarea);
                activate_textarea(&mut self.team_name_textarea);
            }
            SetupScreenState::Confirm => {
                deactivate_textarea(&mut self.user_name_textarea);
                deactivate_textarea(&mut self.team_name_textarea);
            }
        }

        frame.render_widget(
            self.user_name_textarea.widget(),
            menu_layout.menu_option_areas[0],
        );
        frame.render_widget(
            self.team_name_textarea.widget(),
            menu_layout.menu_option_areas[1],
        );

        // submit message
        // TODO: make this a button?
        // TODO: add an option to go back to the slash screen
        if self.state == SetupScreenState::Confirm {
            if validate_textarea(&mut self.user_name_textarea)
                && validate_textarea(&mut self.team_name_textarea)
            {
                frame.render_widget(
                    Paragraph::new("Ready to begin? (Enter)").centered(),
                    menu_layout.menu_option_areas[2],
                );
            } else {
                frame.render_widget(
                    Paragraph::new("Enter valid values to continue.").centered(),
                    menu_layout.menu_option_areas[2],
                );
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // setup form navigation
                KeyCode::Up | KeyCode::BackTab => {
                    self.set_state(self.state.prev());
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.set_state(self.state.next());
                }
                KeyCode::Enter => match self.state {
                    SetupScreenState::Confirm => {
                        if validate_textarea(&mut self.user_name_textarea)
                            && validate_textarea(&mut self.team_name_textarea)
                        {
                            return Some(InterfaceCallback::StartGame {
                                game: self.init_game(),
                            });
                        }
                    }
                    _ => self.set_state(self.state.next()),
                },
                _ => {
                    // text area inputs
                    // TODO: only allow alphanumeric keys in text areas
                    match self.state {
                        SetupScreenState::UserName | SetupScreenState::TeamName => {
                            let active_textarea = self.get_active_textarea_mut();
                            if active_textarea.lines()[0].len() < MAX_NAME_LENGTH
                                || key_event.code == KeyCode::Backspace
                                || key_event.code == KeyCode::Left
                                || key_event.code == KeyCode::Right
                                || key_event.code == KeyCode::Delete
                                || key_event.code == KeyCode::End
                                || key_event.code == KeyCode::Home
                            {
                                active_textarea.input(key_event);
                            }
                        }
                        _ => {}
                    }
                }
            };
            validate_textarea(&mut self.user_name_textarea);
            validate_textarea(&mut self.team_name_textarea);
        }
        None
    }

    fn handle_tick_event(&mut self) -> Option<InterfaceCallback> {
        None
    }
}

fn activate_textarea(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    if let Some(block) = textarea.block() {
        textarea.set_block(block.clone().border_style(Color::Blue))
    }
}

fn deactivate_textarea(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_style(Style::default());
    if let Some(block) = textarea.block() {
        textarea.set_block(block.clone().border_style(Style::default()))
    }
}

fn validate_textarea(textarea: &mut TextArea<'_>) -> bool {
    let text = textarea.lines()[0].trim();
    if text.len() < MIN_NAME_LENGTH || text.len() > MAX_NAME_LENGTH {
        textarea.set_style(InputStyle::INPUT_ERROR);
        false
    } else {
        textarea.set_style(InputStyle::INPUT_DEFAULT);
        true
    }
}

fn build_textarea_block(title: String) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Rounded)
        .title(title)
}

fn generate_player_name() -> String {
    loop {
        let name = RandomNameGenerator::init().generate().first_name;
        if name.len() <= MAX_NAME_LENGTH {
            return name;
        }
    }
}
