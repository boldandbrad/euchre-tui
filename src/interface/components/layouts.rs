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
