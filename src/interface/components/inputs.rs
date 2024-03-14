use ratatui::style::{Color, Modifier, Style};

const DEFAULT_INPUT_STYLE: Style = Style {
    fg: None,
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub struct InputStyle;
impl InputStyle {
    pub const INPUT_DEFAULT: Style = DEFAULT_INPUT_STYLE;
    pub const INPUT_ERROR: Style = DEFAULT_INPUT_STYLE.fg(Color::Red);
}
