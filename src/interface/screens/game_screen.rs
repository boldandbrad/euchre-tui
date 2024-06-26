use crate::interface::{
    components::popups::centered_popup_area, interface_callback::InterfaceCallback,
    layouts::game::GameLayout, screens::Screen,
};
use crate::{
    engine::{
        card::Card,
        game::Game,
        player::Player,
        table::{Seat, SEAT_VARIANTS},
    },
    interface::components::cards::bottom_player_cards,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
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

        // render score boards
        frame.render_widget(
            Paragraph::new(self.game.user_team.name.clone()).alignment(Alignment::Center),
            game_layout.left_score_area.team_name_area,
        );
        frame.render_widget(
            Paragraph::new(self.game.user_team.game_score.to_string()).alignment(Alignment::Center),
            game_layout.left_score_area.game_score_area,
        );
        frame.render_widget(
            Paragraph::new(self.game.user_team.hand_score.to_string()).alignment(Alignment::Center),
            game_layout.left_score_area.hand_score_area,
        );
        frame.render_widget(
            Paragraph::new(self.game.opp_team.name.clone()).alignment(Alignment::Center),
            game_layout.right_score_area.team_name_area,
        );
        frame.render_widget(
            Paragraph::new(self.game.opp_team.game_score.to_string()).alignment(Alignment::Center),
            game_layout.right_score_area.game_score_area,
        );
        frame.render_widget(
            Paragraph::new(self.game.user_team.hand_score.to_string()).alignment(Alignment::Center),
            game_layout.right_score_area.hand_score_area,
        );

        // render player areas
        for seat in SEAT_VARIANTS {
            let player: &Player = self.game.get_player_in_seat(seat.clone());
            let player_area = game_layout.get_player_area_by_seat(seat.clone());
            frame.render_widget(Paragraph::new(player.name.as_str()), player_area.name_area);
            // TODO: once all the card rendering logic is sorted out, this will be fully dynamic
            match seat {
                Seat::Bottom => {
                    frame.render_widget(
                        bottom_player_cards(player.hand.clone()),
                        game_layout.bottom_player_area.hand_area,
                    );
                }
                _ => {
                    frame.render_widget(build_card_lines(&player.hand), player_area.hand_area);
                }
            }
        }

        // render table area
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Table"),
            game_layout.table_area,
        );

        // TODO: eventually remove debug area or hide behind cli flag
        // render debug area
        let debug_block = Block::default().title("Debug");
        let debug_area = debug_block.inner(game_layout.debug_area);
        frame.render_widget(debug_block, game_layout.debug_area);
        frame.render_widget(
            Text::from(vec![
                Line::from("Tick Count: ".to_string() + self.tick_count.to_string().as_str()),
                Line::from("Game State: ".to_string() + self.game.state.to_string().as_str()),
                Line::from(
                    "Current Player Seat: ".to_string()
                        + self.game.current_player_seat.to_string().as_str(),
                ),
                Line::from(
                    "Current Player Name: ".to_string() + self.game.current_player().name.as_str(),
                ),
                Line::from(
                    "Dealer Seat: ".to_string() + self.game.dealer_seat.to_string().as_str(),
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
        if !self.is_paused {
            self.tick_count = self.game.handle_game_tick(self.tick_count);
        }
        None
    }
}

fn build_card_lines(hand: &[Card]) -> Text {
    let mut lines = vec![];
    for card in hand.iter().copied() {
        let line = Line::from(card.get_name());
        lines.push(line);
    }
    Text::from(lines)
}
