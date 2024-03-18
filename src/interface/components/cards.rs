use crate::engine::card::Card;
use ratatui::{
    text::{Line, Text},
    widgets::Paragraph,
};

pub const VERTICAL_CARD_LEFT_EDGE: [&str; 7] = ["╭", "│", "│", "│", "│", "│", "╰"];
pub const VERTICAL_CARD_RIGHT_EDGE: [&str; 7] = ["╮", "│", "│", "│", "│", "│", "╯"];

// TODO: figure out how to make these dynamic based on the screen size
pub const CARD_WIDTH: usize = 9;
pub const CARD_HEIGHT: usize = 7;

// pub fn horizontal_cards(cards: Vec<Card>, splayed: bool, show_values: bool) {}

// pub fn vertical_cards(show_values: bool) {}

pub fn card_area(card: Card) -> Vec<String> {
    let card_area = vec![
        "─".repeat(CARD_WIDTH),
        format!(" {}{}", card.face.get_symbol(), " ".repeat(CARD_WIDTH - 2)),
        " ".repeat(CARD_WIDTH),
        format!(
            "{}{}{}",
            " ".repeat(CARD_WIDTH / 2),
            card.suit.get_symbol(),
            " ".repeat(CARD_WIDTH / 2),
        ),
        " ".repeat(CARD_WIDTH),
        format!("{}{} ", " ".repeat(CARD_WIDTH - 2), card.face.get_symbol()),
        "─".repeat(CARD_WIDTH),
    ];
    card_area
}

pub fn bottom_player_cards(cards: Vec<Card>) -> Paragraph<'static> {
    let mut card_areas: Vec<Vec<String>> = vec![];
    for card in cards {
        card_areas.push(card_area(card));
    }

    let mut hand_area = vec!["".to_string(); CARD_HEIGHT];
    for (_idx, card_area) in card_areas.iter().enumerate() {
        for (idx, line) in card_area.iter().enumerate() {
            hand_area[idx].push_str(VERTICAL_CARD_LEFT_EDGE[idx]);
            hand_area[idx].push_str(&line);
            hand_area[idx].push_str(VERTICAL_CARD_RIGHT_EDGE[idx]);
        }
    }

    string_vec_to_paragraph(hand_area)
}

// pub fn top_player_cards(cards: Vec<Card>) -> Paragraph<'static> {
//     string_vec_to_paragraph(cards.into_iter().map(card_area).flatten().collect())
// }

pub fn string_vec_to_paragraph(strings: Vec<String>) -> Paragraph<'static> {
    let mut lines = Vec::new();
    for line in strings {
        lines.push(Line::from(line))
    }
    Paragraph::new(Text::from(lines)).alignment(ratatui::layout::Alignment::Center)
}
