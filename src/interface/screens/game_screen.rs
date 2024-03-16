use crate::engine::game::Game;
use crate::engine::table::player::Player;
use crate::interface::{
    components::{layouts::GameLayout, popups::centered_popup_area},
    interface_callback::InterfaceCallback,
    screens::Screen,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
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
}

impl Screen for GameScreen {
    // render the game screen to the frame
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let game_layout = GameLayout::new(frame);
        // TODO: make reusable components for score boards and player areas
        // TODO: implement score boards

        // left score board
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Left Score"),
            game_layout.left_score_area,
        );

        // top player area
        let partner = match self.game.teams.first().unwrap().players.last() {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        frame.render_widget(build_card_lines(&partner), game_layout.top_player_area);

        // right score board
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Right Score"),
            game_layout.right_score_area,
        );

        // left player area
        let left_player = match self.game.teams.last().unwrap().players.first() {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        frame.render_widget(build_card_lines(&left_player), game_layout.left_player_area);

        // table area
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Table"),
            game_layout.table_area,
        );

        // right player area
        let right_player = match self.game.teams.last().unwrap().players.last() {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        frame.render_widget(
            build_card_lines(&right_player),
            game_layout.right_player_area,
        );

        // bottom player panel (user)
        let user = match self.game.teams.first().unwrap().players.first() {
            Some(player) => player.clone(),
            _ => panic!("player not here"),
        };
        frame.render_widget(build_card_lines(&user), game_layout.bottom_player_area);

        // TODO: eventually remove debug area or hide behind cli flag
        // debug area
        let debug_block = Block::default().title("Debug");
        let debug_area = debug_block.inner(game_layout.debug_area);
        frame.render_widget(debug_block, game_layout.debug_area);
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

        // io area
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Input/Msg Block"),
            game_layout.msg_input_area,
        );

        // pause menu
        if self.is_paused {
            let block = Block::default()
                .title("Pause Menu")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let menu_container = centered_popup_area(40, 30, frame.size());
            let menu_area = block.inner(menu_container);
            frame.render_widget(Clear, menu_container);
            frame.render_widget(block, menu_container);
            frame.render_widget(Paragraph::new("Options coming soon..."), menu_area);
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<InterfaceCallback> {
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
