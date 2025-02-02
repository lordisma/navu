use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::{Position, Rect}, Frame};

pub trait UIComponent {
    fn key(&mut self, key: KeyEvent);
    fn click(&mut self, event: MouseEvent);

    fn contains(&self, position: Position) -> bool {
        self.area().map_or(false, |area| area.contains(position))
    }

    fn area(&self) -> Option<Rect>;
    fn render(&mut self, frame: &mut Frame);
}