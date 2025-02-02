use crate::ui::state::UIComponent;
use crate::error::Error;

use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::style::{Modifier, Style};
use ratatui::layout::{Margin, Rect};
use ratatui::Frame;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use std::path::Path;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Template {
    showing_document: bool,
    cursor: usize,

    document: Vec<String>,
    document_path: Option<String>,
    pub(crate) area: Option<Rect>,
}

impl Into<ScrollbarState> for &mut Template {
    fn into(self) -> ScrollbarState {
        ScrollbarState::default()
            .content_length(self.document.len())
            .viewport_content_length(self.area.unwrap().height.into())
            .position(self.cursor)
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            showing_document: false,
            area: None,
            document: vec![],
            cursor: 0,
            document_path: None,
        }
    }
}

impl Template {
    fn toggle_document(&mut self) {
        self.showing_document = !self.showing_document;

        if !self.showing_document {
            self.clear_document();
        }
    }

    fn clear_document(&mut self) {
        self.document.clear();
        self.cursor = 0;
        self.document_path = None;
    }

    fn scroll(&self) -> Option<Scrollbar> {
        if self.document.len() <= (self.area.unwrap().height - 2_u16) as usize {
            return None;
        }

        Option::Some(Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None))
    }

    fn text(&self) -> String {
        let window_size = self.area.unwrap().height as usize;
        let content_size = self.document.len();

        if content_size <= window_size {
            return self.document.join("\n");
        }

        let cursor_start = self.cursor;
        let cursor_end = self.cursor.saturating_add(window_size);

        let (from, to) = if cursor_end > content_size {
            (content_size.saturating_sub(window_size), content_size.saturating_sub(1))
        } else {
            (cursor_start, cursor_end)
        };

        self.document[from..to].join("\n")
    }

    pub fn show_document(&mut self, path: &Path) {
        let maybe_document = read_to_string(path)
            .map(|content| content.lines().map(|line| line.to_string()).collect())
            .map_err(|err| Error::Unknown(err));

        match maybe_document {
            Ok(document) => {
                self.document = document;
                self.cursor = 0;
                self.document_path = Some(path.to_string_lossy().to_string());
                if !self.showing_document {
                    self.toggle_document();
                }
            },
            Err(err) => {
                self.clear_document();
                println!("Error: {}", err);
            }
        }
    }
}

impl UIComponent for Template {
    fn key(&mut self, _key: KeyEvent) {
        self.show_document(Path::new("Cargo.lock"));
    }

    fn click(&mut self, event: MouseEvent) {
        if !self.showing_document {
            return;
        }

        match event.kind {
            MouseEventKind::ScrollUp => {
                if self.cursor > 0 {
                    self.cursor = self.cursor.saturating_sub(1);
                }
            },
            MouseEventKind::ScrollDown => {
                self.cursor = self.cursor.saturating_add(1);
                if self.cursor >= self.document.len() {
                    self.cursor = self.document.len() - 1;
                }
            },
            _ => {}
        }
    }

    fn area(&self) -> Option<Rect> {
        self.area
    }

    fn render(&mut self, frame: &mut Frame) {
        let block = if self.showing_document {
            let title = self.document_path.clone().unwrap();
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{}", title))
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .border_style(Style::default())
        } else {
            Block::default()
                .borders(Borders::ALL)
                .title("Template")
                .border_style(Style::default())
        };

        frame.render_widget(block, self.area.unwrap());
        
        if self.showing_document && self.document.len() > 0 {
            let mut state = self.into();
            if let Some(scrollbar) = self.scroll() {
                frame.render_stateful_widget(scrollbar, self.area.unwrap(), &mut state);
            }

            let textarea = Text::raw(self.text());
            frame.render_widget(textarea, self.area.unwrap().inner(Margin::new(4, 2)));
        }
    }
}