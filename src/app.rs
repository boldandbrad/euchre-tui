use crate::engine::game::GameState;
use crate::interface::{interface::Interface, interface_callback::InterfaceCallback};
use crate::structs::{Card, Hand, Rank, Seat, Suit};
use crate::table::{
    player::{Bot, Human, Player},
    team::Team,
};
use crate::tui::Tui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    io::{stdout, Result},
    time::Duration,
};

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
    pub running: bool,
    pub game_state: GameState,
    pub interface: Interface,
}

impl App {
    pub fn new() -> Self {
        let game_state = initialize_game(3);
        let interface = Interface::new();
        App {
            running: true,
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
        while self.running {
            // draw tui
            tui.draw(self)?;

            // handle events
            if let Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    // exit app on `Esc` or `Ctrl-C`
                    KeyCode::Esc => self.exit()?,
                    KeyCode::Char('c') | KeyCode::Char('C')
                        if key_event.modifiers == KeyModifiers::CONTROL =>
                    {
                        self.exit()?;
                    }
                    _ => {
                        self.handle_key_event(key_event)?;
                    }
                }
            }

            // TODO: is this necessary?
            // wait for a short moment before the next iteration
            std::thread::sleep(Duration::from_millis(16));
        }
        tui.exit()?;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let callback = self.interface.handle_key_event(key_event);
        match callback {
            Some(InterfaceCallback::Exit) => self.exit(),
            _ => Ok(()),
        }
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let _ = self.interface.render(frame, &self.game_state);
    }

    pub fn exit(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }
}
