use crate::interface::traits::Screen;
use crate::structs::GameState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{block::Block, Borders},
    Frame,
};
use std::io::Result;

pub struct GameScreen {
    game_state: GameState,
}

impl GameScreen {
    pub fn new(game_state: &GameState) -> Self {
        GameScreen {
            game_state: game_state.clone(),
        }
    }

    pub fn build_top_player_panel() -> Result<()> {
        Ok(())
    }

    pub fn build_left_player_panel() -> Result<()> {
        Ok(())
    }

    pub fn build_right_player_panel() -> Result<()> {
        Ok(())
    }

    pub fn build_bottom_player_panel() -> Result<()> {
        Ok(())
    }
}

impl Screen for GameScreen {
    fn render(&mut self, frame: &mut Frame) {
        let user = self.game_state.user_team.players.get(0);

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
        let layout_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[0]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Left Score"),
            layout_top[0],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Top Player"),
            layout_top[1],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Right Score"),
            layout_top[2],
        );

        // Middle Row
        let layout_mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[1]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Left Player"),
            layout_mid[0],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Table"),
            layout_mid[1],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Right Player"),
            layout_mid[2],
        );

        // Bottom Row
        let layout_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[2]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Blank"),
            layout_bot[0],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Bottom Player"),
            layout_bot[1],
        );
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Blank"),
            layout_bot[2],
        );

        // Input Row
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Input/Msg Block"),
            layout[3],
        );
    }
}
