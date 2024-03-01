use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crossterm::{execute, style};
use rand::seq::SliceRandom;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    widgets::{block::Block, Borders},
    Frame,
};
use std::io::{stdout, Result};
use std::time::Duration;

// Define card suits
#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Define card ranks
#[derive(Debug, Copy, Clone)]
enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

// Define a card
#[derive(Debug, Copy, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    // fn new(rank: Rank, suit: Suit) -> Self {
    //     Card { rank, suit }
    // }
}

// Define a player's hand
struct Hand {
    cards: Vec<Card>,
}

// Define the game state
struct GameState {
    current_screen: CurrentScreen,
    player_hand: Hand,
    cpu_hands: Vec<Hand>,
}

enum CurrentScreen {
    Title,
    GameTable,
    Settings,
}

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

    GameState {
        current_screen: CurrentScreen::GameTable,
        player_hand,
        cpu_hands,
    }
}

// Draw a card at the specified position
fn draw_card(card: Card, x: usize, y: usize) -> Result<()> {
    let suit_symbol = match card.suit {
        Suit::Hearts => '♥',
        Suit::Diamonds => '♦',
        Suit::Clubs => '♣',
        Suit::Spades => '♠',
    };
    let rank_symbol = match card.rank {
        Rank::Nine => '9',
        Rank::Ten => 'T',
        Rank::Jack => 'J',
        Rank::Queen => 'Q',
        Rank::King => 'K',
        Rank::Ace => 'A',
    };
    execute!(
        stdout(),
        crossterm::cursor::MoveTo(x as u16, y as u16),
        style::Print(rank_symbol),
        style::Print(suit_symbol)
    )?;
    Ok(())
}

fn render_tui(frame: &mut Frame, game_state: &GameState) {
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

fn main() -> Result<()> {
    // Initialize the terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.hide_cursor()?;

    // Initialize game state with one CPU player
    let mut game_state = initialize_game(3);

    // Main application loop
    loop {
        // Clear the terminal
        terminal.clear()?;

        // Draw player's hand
        // for (i, &card) in game_state.player_hand.cards.iter().enumerate() {
        //     draw_card(card, i * 3, 0)?;
        // }

        // Draw CPU player's hand
        // for (i, hand) in game_state.cpu_hands.iter().enumerate() {
        //     for (j, &card) in hand.cards.iter().enumerate() {
        //         draw_card(card, j * 3, (i + 2) * 2)?;
        //     }
        // }

        // Display instructions
        // terminal.draw(|frame| {
        //     let area = frame.size();
        //     frame.render_widget(
        //         Paragraph::new("Hello, world! (press 'q' to quit)").white(),
        //         area,
        //     );
        // })?;

        terminal.draw(|frame| render_tui(frame, &game_state))?;

        // Handle events
        if let Event::Key(event) = crossterm::event::read()? {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Char('q') => break, // Quit if 'q' is pressed
                    _ => {}
                }
            }
        }

        // Wait for a short moment before the next iteration
        std::thread::sleep(Duration::from_millis(16));
    }

    // Cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
