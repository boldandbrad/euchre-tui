use crate::interface::{
    components::{ascii_text::*, layouts::MenuLayout},
    interface_callback::InterfaceCallback,
    screens::{paragraph_from_multiline_string, Screen},
};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::Alignment,
    prelude::Margin,
    style::Stylize,
    widgets::{block::Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: implement ability to traverse menu with arrow keys and select an option with enter
// splash screen repr
#[derive(Default)]
pub struct SplashScreen {
    title: Paragraph<'static>,
    menu_option_text: Vec<String>,
}

impl SplashScreen {
    pub fn new() -> Self {
        let title = paragraph_from_multiline_string(SPLASH_TEXT).green();
        let menu_option_text = vec![
            " New Game (n)".to_string(),
            // " Resume Game (l)".to_string(),
            // " Learn Euchre (e)".to_string(),
            // " Settings (s)".to_string(),
            // " Scores (c)".to_string(),
            // " About (a)".to_string(),
            // " Help (h)".to_string(),
            " Exit (Esc/Ctrl-C)".to_string(),
        ];
        Self {
            title,
            menu_option_text,
        }
    }
}

impl Screen for SplashScreen {
    // render the splash screen to the frame
    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        let menu_layout = MenuLayout::new(frame, self.menu_option_text.len());

        // app title
        frame.render_widget(self.title.clone(), menu_layout.header_area);

        // app version
        frame.render_widget(
            Paragraph::new(format!("v{VERSION}")).alignment(Alignment::Center),
            menu_layout.sub_header_area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
        );

        // app menu
        for i in 0..self.menu_option_text.len() {
            let menu_option = Paragraph::new(self.menu_option_text[i].clone()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

            frame.render_widget(menu_option, menu_layout.menu_option_areas[i]);
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Option<InterfaceCallback> {
        if key_event.kind == KeyEventKind::Press {
            #[allow(clippy::single_match)] // more will be added to this in the future
            match key_event.code {
                KeyCode::Char('n') => return Some(InterfaceCallback::SetupNewGame),
                _ => {}
            }
        }
        None
    }

    fn handle_tick_event(&mut self) -> Option<InterfaceCallback> {
        None
    }
}
