use crate::data::state::SharedState;
use crate::ui::pages::PageType::Wallet;
use super::pages::{Page, PageType, DashboardPage, DetailPage};
use super::pages::WalletPage;
pub struct App {
    pub current_page: PageType,
    pub dashboard: DashboardPage,
    pub detail_page: Option<DetailPage>,
    pub should_quit: bool,
    pub wallet_page: Option<WalletPage>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: PageType::Dashboard,
            dashboard: DashboardPage::new(),
            detail_page: None,
            should_quit: false,
            wallet_page: None
        }
    }

    pub fn navigate_to(&mut self, page: PageType) {
        self.current_page = page;
    }

    pub fn navigate_to_detail(&mut self, title: String, content: String, identifier: String) {
        self.detail_page = Some(DetailPage::new(title, content, identifier));
        self.current_page = PageType::Detail;
    }

    pub fn navigate_to_wallet(&mut self, title: String) {
        self.wallet_page = Some(WalletPage::new(title));
        self.current_page = PageType::Wallet;
    }

    pub fn go_back(&mut self) {
        match self.current_page {
            PageType::Detail => {
                self.current_page = PageType::Dashboard;
                self.detail_page = None;
            }
            PageType::Wallet => {
                self.current_page = PageType::Dashboard;
                self.wallet_page = None;
            }
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}