use ratatui::widgets::Row;

use ratatui::style::{Color, Stylize};
use ratatui::text::Text;

#[derive(Debug, Clone)]
pub struct Command{
    pub command: String,
    pub tags: Vec<String>, 
    pub description: String
}

impl<'a> Into<Row<'a>> for Command {
    fn into(self) -> Row<'a> {
        let tags =  if self.tags.is_empty() {
            Text::from("No tags").fg(Color::Indexed(75))
        } else {
            let tags = self.tags.join(", ");
            Text::from(tags)
                .bold()
                .fg(Color::Indexed(220))
        };

        let command = Text::from(self.command)
            .bold()
            .fg(Color::Indexed(234));

        let description = Text::from(self.description)
            .fg(Color::Indexed(234));

        Row::new(vec![command, tags, description])
    }
}

impl From<Vec<String>> for Command {
    fn from(mut vec: Vec<String>) -> Self {
        let command = vec.pop().unwrap_or_default();
        let description = vec
            .last()
            .cloned()
            .unwrap_or_default();

        vec.truncate(vec.len() - 1); 
            
        Self {
            command, 
            tags: vec, 
            description
        }
    }
}