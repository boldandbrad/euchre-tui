use crate::engine::game::GameState;
use crate::interface::interface::Interface;
use crate::structs::{Card, Hand, Rank, Seat, Suit};
use crate::table::player::{Bot, Human, Player};
use crate::table::team::Team;
use crate::tui::Tui;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use rand::seq::SliceRandom;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::time::Duration;

// Initialize the game state with a specified number of CPU players
fn initialize_game(num_cpus: usize) -> GameState {
    // Create and shuffle a deck of cards
    let mut deck = Vec::new();
    for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        for &rank in &[
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ] {
            deck.push(Card { rank, suit });
        }
    }
    // Shuffle the deck
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // Deal hands to player and CPU players
    let player_hand = Hand {
        cards: deck.drain(0..5).collect(),
    };
    let mut cpu_hands = Vec::new();
    for _ in 0..num_cpus {
        cpu_hands.push(Hand {
            cards: deck.drain(0..5).collect(),
        });
    }

    let user = Human::new("Brad".to_string(), Seat::Bottom, player_hand);
    let user_partner = Bot::new("Morgan".to_string(), Seat::Top, cpu_hands.remove(0));
    let user_team = Team::new(
        "Team A".to_string(),
        vec![Player::Human(user), Player::Bot(user_partner)],
    );

    let opponent_1 = Bot::new("Kyle".to_string(), Seat::Left, cpu_hands.remove(0));
    let opponent_2 = Bot::new("Ryan".to_string(), Seat::Right, cpu_hands.remove(0));
    let opposing_team = Team::new(
        "Team B".to_string(),
        vec![Player::Bot(opponent_1), Player::Bot(opponent_2)],
    );

    GameState {
        user_team,
        opposing_team,
        deck,
    }
}

// application repr
pub struct App {
    pub game_state: GameState,
    pub interface: Interface,
}

impl App {
    pub fn new() -> Self {
        let game_state = initialize_game(3);
        let interface = Interface::new();
        App {
            game_state,
            interface,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let terminal: Terminal<CrosstermBackend<std::io::Stdout>> =
            Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut tui = Tui::new(terminal);
        tui.init()?;

        // main application loop
        loop {
            // draw tui
            tui.draw(self)?;

            // handle events
            if let Event::Key(event) = crossterm::event::read()? {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char('q') => break, // quit if 'q' is pressed
                        _ => {}
                    }
                }
            }

            // wait for a short moment before the next iteration
            std::thread::sleep(Duration::from_millis(16));
        }
        tui.exit()?;
        Ok(())
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let _ = self.interface.render(frame, &self.game_state);
    }
}
