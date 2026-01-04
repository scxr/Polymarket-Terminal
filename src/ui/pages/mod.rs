mod dashboard;
mod detail;
mod wallet;



pub use dashboard::DashboardPage;
pub use detail::DetailPage;
pub use wallet::WalletPage;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use crate::data::state::SharedState;
use super::app::App;

#[derive(Clone, Copy, PartialEq)]
pub enum PageType {
    Dashboard,
    Detail,
    Wallet,
}

pub enum PageAction {
    None,
    NavigateToDetail { title: String, content: String, identifier: String },
    NavigateToWallet { title: String },
    GoBack,
    Quit,
}

pub trait Page {
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &SharedState);
    fn handle_input(&mut self, key: KeyEvent, state: &SharedState) -> PageAction;

}