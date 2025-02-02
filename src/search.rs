use ratatui::{
    buffer::Buffer,
    prelude::Rect, 
    style::{Color, Style},
    widgets::{Block, BorderType, Widget},
    
};
use tui_textarea::{Input, TextArea};

#[derive(Debug)]
pub struct SearchWidget<'a> {
    text_area: TextArea<'a>,
}

impl SearchWidget<'_> {
    pub fn new() -> Self {
        let block = Block::default()
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::DarkGray));
        
        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Search");
        text_area.set_block(block);
        
        Self {
            text_area,
        }
    }

    pub fn input(&mut self, input: Input) {
        self.text_area.input(input);
    }
}

impl Widget for SearchWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {
        self.text_area.render(area, buf);
    }
}
