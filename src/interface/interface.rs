// use crate::interface::game_screen::GameScreen;
use crate::game::player::Player;
use crate::structs::GameState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{block::Block, Borders},
    Frame,
};

#[derive(Default)]
pub enum InterfaceState {
    // TODO: make splash default in the future
    Splash,
    Menu,
    NewGame,
    #[default]
    GameTable,
}

pub struct Interface {
    state: InterfaceState,
    // pub game_screen: GameScreen,
}

impl Interface {
    pub fn new() -> Self {
        // let game_screen = GameScreen::new();
        Self {
            state: InterfaceState::default(),
            // game_screen,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, game_state: &GameState) {
        match self.state {
            InterfaceState::Splash => {}
            InterfaceState::Menu => {}
            InterfaceState::NewGame => {}
            InterfaceState::GameTable => {
                let player = game_state.user_team.players.get(0);
                let user = match player {
                    Some(Player::Human(value)) => value.clone(),
                    _ => panic!("player not here"),
                };

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
                    Block::new().borders(Borders::ALL).title(user.name),
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
    }
}

// Draw a card at the specified position
// fn draw_card(card: Card, x: usize, y: usize) -> Result<()> {
//     let suit_symbol = match card.suit {
//         Suit::Hearts => '♥',
//         Suit::Diamonds => '♦',
//         Suit::Clubs => '♣',
//         Suit::Spades => '♠',
//     };
//     let rank_symbol = match card.rank {
//         Rank::Nine => '9',
//         Rank::Ten => 'T',
//         Rank::Jack => 'J',
//         Rank::Queen => 'Q',
//         Rank::King => 'K',
//         Rank::Ace => 'A',
//     };
//     execute!(
//         stdout(),
//         crossterm::cursor::MoveTo(x as u16, y as u16),
//         style::Print(rank_symbol),
//         style::Print(suit_symbol)
//     )?;
//     Ok(())
// }
