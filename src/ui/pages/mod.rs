mod dashboard;
mod detail;

pub use dashboard::DashboardPage;
pub use detail::DetailPage;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use crate::data::state::SharedState;
use super::app::App;

#[derive(Clone, Copy, PartialEq)]
pub enum PageType {
    Dashboard,
    Detail,
}

pub enum PageAction {
    None,
    NavigateToDetail { title: String, content: String, identifier: String },
    GoBack,
    Quit,
}

pub trait Page {
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &SharedState);
    fn handle_input(&mut self, key: KeyEvent, state: &SharedState) -> PageAction;
}