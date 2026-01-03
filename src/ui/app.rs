use crate::data::state::SharedState;
use super::pages::{Page, PageType, DashboardPage, DetailPage};

pub struct App {
    pub current_page: PageType,
    pub dashboard: DashboardPage,
    pub detail_page: Option<DetailPage>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: PageType::Dashboard,
            dashboard: DashboardPage::new(),
            detail_page: None,
            should_quit: false,
        }
    }

    pub fn navigate_to(&mut self, page: PageType) {
        self.current_page = page;
    }

    pub fn navigate_to_detail(&mut self, title: String, content: String) {
        self.detail_page = Some(DetailPage::new(title, content));
        self.current_page = PageType::Detail;
    }

    pub fn go_back(&mut self) {
        match self.current_page {
            PageType::Detail => {
                self.current_page = PageType::Dashboard;
                self.detail_page = None;
            }
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}