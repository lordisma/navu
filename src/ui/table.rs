use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{
    layout::{Constraint, Flex}, 
    prelude::Rect, 
    style::{Color, Style}, 
    widgets::{
        Block, 
        BorderType, 
        Row, 
        Scrollbar, 
        ScrollbarOrientation, 
        ScrollbarState, 
        Table, 
        TableState
    }, Frame
};

use crate::ui::state::UIComponent;

#[derive(Debug, Clone)]
pub struct CommandsTable<'a, T: Into<Row<'a>>> {
    offset: usize,
    cursor: Option<usize>,

    content: Vec<T>,
    pub(crate) commands_in_view: usize,
    
    pub(crate) area: Option<Rect>,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Into<Row<'a>> + Clone> Into<TableState> for &mut CommandsTable<'a, T> {
    fn into(self) -> TableState {
        let mut state = TableState::default();
        *state.offset_mut() = self.offset;
        *state.selected_mut() = self.cursor;

        state
    }
}

impl<'a, T: Into<Row<'a>> + Clone> Into<ScrollbarState> for &mut CommandsTable<'a, T> {
    fn into(self) -> ScrollbarState {
        ScrollbarState::default()
            .content_length(self.content.len())
            .viewport_content_length(self.commands_in_view)
            .position(self.cursor.unwrap_or(0))
    }
}

impl<'a, T: Into<Row<'a>> + Clone> UIComponent for CommandsTable<'a, T> {
    fn area(&self) -> Option<Rect> {
        self.area
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut table_state = self.into();
        let table = self.table();

        frame.render_stateful_widget(table, self.area.unwrap(), &mut table_state);

        let mut scroll_state = self.into();
        let scroll = self.scroll();
        frame.render_stateful_widget(scroll, self.area.unwrap(), &mut scroll_state);
    }

    fn click(&mut self, _: MouseEvent) {
        // No op
    }

    fn key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Up => {
                if let Some(mut cursor) = self.cursor {
                    cursor = cursor.saturating_sub(1);
                    if cursor == 0 {
                        cursor = 0;
                    }

                    self.cursor = Some(cursor);
                } else {
                    self.cursor = Some(0);
                }
            },
            KeyCode::Down => {
                if let Some(mut cursor) = self.cursor {
                    cursor = cursor.saturating_add(1);
                    if cursor >= self.content.len() {
                        cursor = self.content.len() - 1;
                    }

                    self.cursor = Some(cursor);
                } else {
                    self.cursor = Some(0);
                }
            },
            KeyCode::Enter => {
                // Need to have a reference to the template widget to update the template
                // with the selected command.
            }  
            _ => {}
        }
    }
}

impl<'a, T: Into<Row<'a>> + Clone> CommandsTable<'a, T> {
    pub fn new(rows: Vec<T>) -> Self
    where T: Into<Row<'a>> {
        Self {
            offset: 0,
            cursor: None,
            content: rows,
            commands_in_view: 0,
            area: None,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn rows(&self) -> Vec<Row<'a>> {
        let mut rows = self.content.iter().cloned().map(|row| row.into()).collect::<Vec<Row<'a>>>();

        if let Some(cursor) = self.cursor {
            let selected = rows[cursor]
                .clone()
                .style(Style::default().bg(Color::Indexed(240)));
            rows[cursor] = selected;
        }

        rows
    }

    pub fn table(&self) -> Table<'a> {
        let constraints = vec![
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ];

        return Table::new(self.rows(), constraints)
            .header(
                Row::new(vec!["Command", "Tags", "Description"])
                    .style(Style::default().fg(Color::White).bg(Color::Indexed(240)))
                    .height(1)
            )
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .flex(Flex::SpaceBetween)
            .block(Block::bordered().border_type(BorderType::Thick))
    }

    pub fn scroll(&self) -> Scrollbar {
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
    }
}