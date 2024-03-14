use crate::engine::game::Game;
use crate::interface::{interface_callback::InterfaceCallback, traits::Screen};
use crate::table::player::{Bot, Human, Player};
use crossterm::event::KeyEventKind;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{block::Block, Borders},
    Frame,
};
use std::io::Result;

// TODO: handle in-game pause menu as popup panel here
pub struct GameScreen {
    game: Game,
}

impl GameScreen {
    pub fn new() -> Self {
        GameScreen {
            game: Game::default(),
        }
    }

    pub fn set_game(&mut self, game: Game) {
        self.game = game;
    }

    fn build_top_player_panel(&mut self, player: Bot, frame: &mut Frame, area: Rect) -> Result<()> {
        // Build card lines
        let mut lines = vec![Line::from(player.name), Line::from("")];
        for card in player.hand.cards {
            let line = Line::from(card.get_name());
            lines.push(line);
        }
        frame.render_widget(Text::from(lines), area);
        Ok(())
    }

    fn build_left_player_panel(
        &mut self,
        player: Bot,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        // Build card lines
        let mut lines = vec![Line::from(player.name), Line::from("")];
        for card in player.hand.cards {
            let line = Line::from(card.get_name());
            lines.push(line);
        }
        frame.render_widget(Text::from(lines), area);
        Ok(())
    }

    fn build_right_player_panel(
        &mut self,
        player: Bot,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        // Build card lines
        let mut lines = vec![Line::from(player.name), Line::from("")];
        for card in player.hand.cards {
            let line = Line::from(card.get_name());
            lines.push(line);
        }
        frame.render_widget(Text::from(lines), area);
        Ok(())
    }

    fn build_bottom_player_panel(
        &mut self,
        user: Human,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        // Build card lines
        let mut lines = vec![Line::from(user.name), Line::from("")];
        for card in user.hand.cards {
            let line = Line::from(card.get_name());
            lines.push(line);
        }
        frame.render_widget(Text::from(lines), area);
        Ok(())
    }
}

impl Screen for GameScreen {
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
        let layout_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[0]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Left Score"),
            layout_top[0],
        );

        // Top player area
        let top_player = self.game.user_team.players.get(1);
        let partner = match top_player {
            Some(Player::Bot(value)) => value.clone(),
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
        let left_player = self.game.opposing_team.players.get(0);
        let partner = match left_player {
            Some(Player::Bot(value)) => value.clone(),
            _ => panic!("player not here"),
        };
        self.build_left_player_panel(partner, frame, layout_mid[0])?;

        // Table area
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Table"),
            layout_mid[1],
        );

        // Right player area
        let right_player = self.game.opposing_team.players.get(1);
        let partner = match right_player {
            Some(Player::Bot(value)) => value.clone(),
            _ => panic!("player not here"),
        };
        self.build_right_player_panel(partner, frame, layout_mid[2])?;

        // Bottom Row
        let layout_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout[2]);

        // Bottom player panel (user)
        let bottom_player = self.game.user_team.players.get(0);
        let user = match bottom_player {
            Some(Player::Human(value)) => value.clone(),
            _ => panic!("player not here"),
        };
        self.build_bottom_player_panel(user, frame, layout_bot[1])?;

        // Input Row
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Input/Msg Block"),
            layout[3],
        );
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                _ => {}
            }
        }
        None
    }
}
