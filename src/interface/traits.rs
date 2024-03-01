use ratatui::Frame;

pub trait Screen {
    fn render(&mut self, frame: &mut Frame) {}
}
