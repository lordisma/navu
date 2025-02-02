use ratatui::Frame;

use crate::app::App;

pub mod table;
pub mod state;
pub mod search;
pub mod template;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    app.views(area);
    app.render(frame);
}
