use crate::engine::table::Seat;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct PlayerLayout {
    pub name_area: Rect,
    pub hand_area: Rect,
}

impl PlayerLayout {
    pub fn new(rect: Rect) -> Self {
        let layout_base = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Fill(1)])
            .split(rect);
        Self {
            name_area: layout_base[0],
            hand_area: layout_base[1],
        }
    }
}

pub struct GameLayout {
    pub left_score_area: Rect,
    pub right_score_area: Rect,
    pub top_player_area: PlayerLayout,
    pub left_player_area: PlayerLayout,
    pub right_player_area: PlayerLayout,
    pub bottom_player_area: PlayerLayout,
    pub table_area: Rect,
    pub msg_input_area: Rect,
    pub debug_area: Rect,
}

impl GameLayout {
    pub fn new(frame: &mut Frame) -> Self {
        let layout_base = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(3),
                Constraint::Fill(4),
                Constraint::Fill(3),
                Constraint::Length(3),
            ])
            .split(frame.size());
        let layout_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout_base[0]);
        let layout_mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout_base[1]);
        let layout_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_fills([1, 2, 1]))
            .split(layout_base[2]);
        Self {
            left_score_area: layout_top[0],
            right_score_area: layout_top[2],
            top_player_area: PlayerLayout::new(layout_top[1]),
            left_player_area: PlayerLayout::new(layout_mid[0]),
            right_player_area: PlayerLayout::new(layout_mid[2]),
            bottom_player_area: PlayerLayout::new(layout_bot[1]),
            table_area: layout_mid[1],
            msg_input_area: layout_base[3],
            debug_area: layout_bot[2],
        }
    }

    pub fn get_player_area_by_seat(&self, seat: Seat) -> &PlayerLayout {
        match seat {
            Seat::Bottom => &self.bottom_player_area,
            Seat::Left => &self.left_player_area,
            Seat::Top => &self.top_player_area,
            Seat::Right => &self.right_player_area,
        }
    }
}

// TODO: implement GameTableLayout
// TODO: implement ScoreBoardLayout
