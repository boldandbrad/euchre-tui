use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct MenuLayout {
    pub header_area: Rect,
    pub sub_header_area: Rect,
    pub menu_area: Rect,
    pub menu_option_areas: Vec<Rect>,
}

impl MenuLayout {
    pub fn new(frame: &mut Frame, num_menu_options: usize) -> Self {
        let layout_base = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Fill(4),
            ])
            .split(frame.size());
        let layout_bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(42),
                Constraint::Fill(1),
            ])
            .split(layout_base[2]);
        let layout_menu = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(layout_bottom[1]);
        let layout_menu_options = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3)].repeat(num_menu_options))
            .split(layout_menu[1]);
        Self {
            header_area: layout_base[1],
            sub_header_area: layout_menu[0],
            menu_area: layout_menu[1],
            menu_option_areas: layout_menu_options.to_vec(),
        }
    }
}

pub struct GameLayout {
    pub left_score_area: Rect,
    pub right_score_area: Rect,
    pub top_player_area: Rect,
    pub left_player_area: Rect,
    pub right_player_area: Rect,
    pub bottom_player_area: Rect,
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
            top_player_area: layout_top[1],
            left_player_area: layout_mid[0],
            right_player_area: layout_mid[2],
            bottom_player_area: layout_bot[1],
            table_area: layout_mid[1],
            msg_input_area: layout_base[3],
            debug_area: layout_bot[2],
        }
    }
}

// TODO: implement GameTableLayout
// TODO: implement ScoreBoardLayout
// TODO: implement PlayerAreaLayout(s)
