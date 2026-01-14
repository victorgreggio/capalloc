use crate::ui::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    crate::ui::ui_renderer::render_list(f, state, chunks[0]);
    crate::ui::ui_renderer::render_details(f, state, chunks[1]);
}
