use crate::engine::game::Game;
use crate::interface::{
    components::inputs::InputStyle, interface_callback::InterfaceCallback, screens::Screen,
};
use crossterm::event::{KeyCode, KeyEventKind};
use name_maker::RandomNameGenerator;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{block::Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io::Result;
use tui_textarea::TextArea;

const MIN_INPUT_LENGTH: usize = 3;
const MAX_INPUT_LENGTH: usize = 12;

#[derive(Debug, Default, PartialOrd, PartialEq)]
pub enum SetupState {
    #[default]
    UserName,
    TeamName,
    Confirm,
}

impl SetupState {
    pub fn next_step(&self) -> Self {
        match self {
            SetupState::UserName => SetupState::TeamName,
            SetupState::TeamName => SetupState::Confirm,
            SetupState::Confirm => SetupState::Confirm,
        }
    }

    pub fn prev_step(&self) -> Self {
        match self {
            SetupState::Confirm => SetupState::TeamName,
            SetupState::TeamName => SetupState::UserName,
            SetupState::UserName => SetupState::UserName,
        }
    }
}

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
        user_name_textarea.set_cursor_line_style(Style::default());

        let mut team_name_textarea = TextArea::default();
        team_name_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(text_area_style)
                .title("Team name"),
        );
        team_name_textarea.set_cursor_line_style(Style::default());

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

    fn get_active_textarea_mut(&mut self) -> &mut TextArea<'static> {
        match self.state {
            SetupState::UserName => &mut self.user_name_textarea,
            SetupState::TeamName => &mut self.team_name_textarea,
            SetupState::Confirm => unreachable!(),
        }
    }

    fn build_setup_form(&mut self, frame: &mut Frame, area: Rect) {
        let layout_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(42),
                Constraint::Fill(1),
            ])
            .split(area);

        let layout_form = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(layout_columns[1]);

        // text input areas
        match self.state {
            SetupState::UserName => {
                activate_textarea(&mut self.user_name_textarea);
                deactivate_textarea(&mut self.team_name_textarea);
            }
            SetupState::TeamName => {
                deactivate_textarea(&mut self.user_name_textarea);
                activate_textarea(&mut self.team_name_textarea);
            }
            SetupState::Confirm => {
                deactivate_textarea(&mut self.user_name_textarea);
                deactivate_textarea(&mut self.team_name_textarea);
            }
        }
        frame.render_widget(self.user_name_textarea.widget(), layout_form[0]);
        frame.render_widget(self.team_name_textarea.widget(), layout_form[1]);

        // submit message
        if self.state == SetupState::Confirm {
            if validate_textarea(&mut self.user_name_textarea)
                && validate_textarea(&mut self.team_name_textarea)
            {
                frame.render_widget(
                    Paragraph::new("Ready to begin? (Enter)").centered(),
                    layout_form[2],
                );
            } else {
                frame.render_widget(
                    Paragraph::new("Fix invalid values to continue.").centered(),
                    layout_form[2],
                );
            }
        }
    }

    fn init_game(&mut self) -> Game {
        let rng = RandomNameGenerator::init();
        self.partner_name = rng.generate().first_name;
        self.opp1_name = rng.generate().first_name;
        self.opp2_name = rng.generate().first_name;

        Game::new(
            self.user_name_textarea.lines()[0].trim().to_string(),
            self.partner_name.clone(),
            self.opp1_name.clone(),
            self.opp2_name.clone(),
            self.team_name_textarea.lines()[0].trim().to_string(),
            "Bad Guys".to_string(),
        )
    }
}

impl Screen for SetupScreen {
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Fill(3),
            ])
            .split(frame.size());

        // TODO: make screen header fancier
        // TODO: add a description of the screen and how to use it
        // app title
        frame.render_widget(Paragraph::new("Game Setup").centered(), layout[1]);

        // game setup form
        self.build_setup_form(frame, layout[3]);
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Up | KeyCode::BackTab => {
                    self.set_state(self.state.prev_step());
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.set_state(self.state.next_step());
                }
                KeyCode::Enter => match self.state {
                    SetupState::Confirm => {
                        if validate_textarea(&mut self.user_name_textarea)
                            && validate_textarea(&mut self.team_name_textarea)
                        {
                            return Some(InterfaceCallback::StartGame {
                                game: self.init_game(),
                            });
                        }
                    }
                    _ => self.set_state(self.state.next_step()),
                },
                _ => {
                    // TODO: handle special keys
                    match self.state {
                        SetupState::UserName | SetupState::TeamName => {
                            self.get_active_textarea_mut().input(key_event);
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
}

fn activate_textarea(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
}

fn deactivate_textarea(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_style(Style::default());
}

fn validate_textarea(textarea: &mut TextArea<'_>) -> bool {
    let text = textarea.lines()[0].trim();
    if text.len() < MIN_INPUT_LENGTH {
        textarea.set_style(InputStyle::INPUT_ERROR);
        false
    } else if text.len() > MAX_INPUT_LENGTH {
        textarea.set_style(InputStyle::INPUT_ERROR);
        false
    } else {
        textarea.set_style(InputStyle::INPUT_DEFAULT);
        true
    }
}
