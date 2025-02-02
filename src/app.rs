use std::error;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::Frame;

use crate::commands::Command;
use crate::event::Event;
use crate::ui::{table::CommandsTable, state::UIComponent, search::SearchBox, template::Template};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppFocus {
    Table,
    Search,
    Template,
    None
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// State of the table
    pub table_state: CommandsTable<'a, Command>,
    // /// State of the search bar
    pub search_state: SearchBox<'a>,
    // /// State of the Template view
    pub template_state: Template,
    /// Flag to signal the attention of the user
    /// it will have one value for each widget that
    /// will indicate where the user is currently
    pub attention: AppFocus,
}

impl App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            table_state: CommandsTable::new(vec![
                Command {
                    command: "ls".into(), 
                    description: "List directory contents".into(),
                    tags: vec!["list".into(), "directory".into(), "contents".into()],
                }; 10
            ]),
            search_state: SearchBox::default(),
            template_state: Template::default(),
            attention: AppFocus::None,
        }
    }

    pub fn views(&mut self, area: Rect) {
        let [template, commands, search] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Length(self.search_state.height()),
        ])
        .areas(area);

        self.search_state.area = Some(search);
        self.table_state.area = Some(commands);
        self.template_state.area = Some(template);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn key_handling(&mut self, key: KeyEvent) {
        match self.attention {
            AppFocus::Table => self.table_state.key(key),
            AppFocus::Search => self.search_state.key(key),
            AppFocus::Template => self.template_state.key(key),
            _ => {}
        }
    }

    fn click(&mut self, event: MouseEvent) {
        if self.table_state.contains(Position::new( event.column, event.row )) {
            self.attention = AppFocus::Table;
            self.table_state.click(event);
        } else if self.search_state.contains(Position::new( event.column, event.row )) {
            self.attention = AppFocus::Search;
            self.search_state.click(event);
        } else if self.template_state.contains(Position::new( event.column, event.row )) {
            self.attention = AppFocus::Template;
            self.template_state.click(event);
        } else {
            self.attention = AppFocus::None;
        }
    }

    fn resize(&self, _: u16, _: u16) {
        unimplemented!("Resize not implemented yet");
    }

    pub fn handle(&mut self, event: Event) {
        match event {
            Event::Key(key) 
                if (key.code == KeyCode::Esc) 
                    || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))  
            => {
                self.quit();
            },
            Event::Key(key) => {
                self.key_handling(key);
            },
            Event::Mouse(mouse_event) => {
                self.click(mouse_event);
            },
            Event::Resize(x, y) => {
                self.resize(x, y);
            },
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.table_state.render(frame);
        self.search_state.render(frame);
        self.template_state.render(frame);
    }
}
