use crate::engine::game::Game;
use crate::engine::table::player::Player;
use crate::interface::{interface_callback::InterfaceCallback, screens::Screen};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{block::Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use std::io::Result;

// game screen repr
#[derive(Default)]
pub struct GameScreen {
    game: Game,
    is_paused: bool,
    tick_count: u64,
}

impl GameScreen {
    pub fn new() -> Self {
        GameScreen {
            game: Game::default(),
            is_paused: false,
            tick_count: 0,
        }
    }

    pub fn set_game(&mut self, game: Game) {
        self.game = game;
    }

    fn build_top_player_panel(
        &mut self,
        player: Player,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        frame.render_widget(build_card_lines(&player), area);
        Ok(())
    }

    fn build_left_player_panel(
        &mut self,
        player: Player,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        frame.render_widget(build_card_lines(&player), area);
        Ok(())
    }

    fn build_right_player_panel(
        &mut self,
        player: Player,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        frame.render_widget(build_card_lines(&player), area);
        Ok(())
    }

    fn build_bottom_player_panel(
        &mut self,
        user: Player,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        frame.render_widget(build_card_lines(&user), area);
        Ok(())
    }
}

impl Screen for GameScreen {
    // render the game screen to the frame
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(3),
                Constraint::Fill(4),
                Constraint::Fill(3),
                Constraint::Length(3),
            ])
            .split(frame.size());

        // Top Row
        // TODO: implement score boards
        let layout_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[0]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Left Score"),
            layout_top[0],
        );

        // Top player area
        let top_player = self.game.teams.first().unwrap().players.last();
        let partner = match top_player {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        self.build_top_player_panel(partner, frame, layout_top[1])?;

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Right Score"),
            layout_top[2],
        );

        // Middle Row
        let layout_mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[1]);

        // Left player area
        let left_player = self.game.teams.last().unwrap().players.first();
        let partner = match left_player {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        self.build_left_player_panel(partner, frame, layout_mid[0])?;

        // Table area
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Table"),
            layout_mid[1],
        );

        // Right player area
        let right_player = self.game.teams.last().unwrap().players.last();
        let partner = match right_player {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        self.build_right_player_panel(partner, frame, layout_mid[2])?;

        // Bottom Row
        let layout_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[2]);

        // Bottom player panel (user)
        let bottom_player = self.game.teams.first().unwrap().players.first();
        let user = match bottom_player {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        self.build_bottom_player_panel(user, frame, layout_bot[1])?;

        // TODO: eventually remove debug area or hide behind cli flag
        // Debug area
        let debug_block = Block::default().title("Debug");
        let debug_area = debug_block.inner(layout_bot[2]);
        frame.render_widget(debug_block, layout_bot[2]);
        frame.render_widget(
            Text::from(vec![
                Line::from("Tick Count: ".to_string() + self.tick_count.to_string().as_str()),
                Line::from(
                    "Current Player Index: ".to_string()
                        + self.game.current_player_index.to_string().as_str(),
                ),
                Line::from(
                    "Current Player Name: ".to_string() + self.game.current_player().name.as_str(),
                ),
            ]),
            debug_area,
        );

        // Input Row
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Input/Msg Block"),
            layout[3],
        );

        if self.is_paused {
            let block = Block::default()
                .title("Pause Menu")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let menu_container = centered_rect(60, 20, frame.size());
            let menu_area = block.inner(menu_container);
            frame.render_widget(Clear, menu_container);
            frame.render_widget(block, menu_container);
            frame.render_widget(Paragraph::new("Options coming soon..."), menu_area);
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            #[allow(clippy::match_single_binding)] // this will be populated in the future
            match key_event.code {
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    self.is_paused = !self.is_paused;
                }
                _ => {}
            }
        }
        None
    }

    fn handle_tick_event(&mut self) -> Option<InterfaceCallback> {
        match self.is_paused {
            false => {
                if self.tick_count >= 20 {
                    self.game.next_turn();
                    self.tick_count = 0
                } else {
                    self.tick_count += 1
                }
            }
            true => {}
        }
        None
    }
}

fn build_card_lines(player: &Player) -> Text {
    let mut lines = vec![Line::from(player.name.clone()), Line::from("")];
    for card in player.hand.clone() {
        let line = Line::from(card.get_name());
        lines.push(line);
    }
    Text::from(lines)
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
