use std::fmt::Display;

use crate::ui::state::UIComponent;

use ratatui::widgets::{Block, Borders};
use ratatui::style::{Color, Style};
use ratatui::layout::Rect;
use ratatui::Frame;
use crossterm::event::{KeyEvent, MouseEvent};
use tui_textarea::{TextArea, CursorMove, Input, Key};

#[derive(Debug)]
pub struct SearchBox<'a> {
    textarea: TextArea<'a>,

    pub(crate) area: Option<Rect>,
}

impl Default for SearchBox<'_> {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::default().borders(Borders::ALL).title("Search"));
        Self {
            textarea,
            area: None,
        }
    }
}

impl SearchBox<'_> {
    pub fn clear(&mut self) {
        // Remove input for next search. Do not recreate `self.textarea` instance to keep undo history so that users can
        // restore previous input easily.
        self.textarea.move_cursor(CursorMove::End);
        self.textarea.delete_line_by_head();
    }

    pub fn text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub fn height(&self) -> u16 {
        3
    }

    fn input(&mut self, input: Input) -> Option<&'_ str> {
        match input {
            Input {
                key: Key::Enter, ..
            }
            | Input {
                key: Key::Char('m'),
                ctrl: true,
                ..
            } => None, // Disable shortcuts which inserts a newline. See `single_line` example
            input => {
                let modified = self.textarea.input(input);
                modified.then(|| self.textarea.lines()[0].as_str())
            }
        }
    }

    pub fn set_error(&mut self, err: Option<impl Display>) {
        let b = if let Some(err) = err {
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Search: {}", err))
                .style(Style::default().fg(Color::Red))
        } else {
            Block::default().borders(Borders::ALL).title("Search")
        };
        self.textarea.set_block(b);
    }
}

impl UIComponent for SearchBox<'_> {
    fn key(&mut self, key: KeyEvent) {
        self.input(Input::from(key));
    }

    fn click(&mut self, _: MouseEvent) {}

    fn area(&self) -> Option<Rect> {
        self.area
    }

    fn render(&mut self, frame: &mut Frame) {
        if let Some(area) = self.area {
            frame.render_widget(&self.textarea, area);
        }
    }
}