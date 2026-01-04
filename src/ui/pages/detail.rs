use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::Stylize;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::data::get_market::get_market_from_slug;
use crate::data::state::SharedState;
use crate::data::types::MarketSpecificDetails;
use crate::actions::buy::buy_yes;
use super::{Page, PageAction};

pub struct DetailPage {
    pub title: String,
    pub content: String,
    pub scroll_offset: u16,
    pub id: String,
    pub market_data: Option<MarketSpecificDetails>,
    pub last_fetch: Option<Instant>,
    pub is_loading: bool,
    pub error: Option<String>,
}

impl DetailPage {
    pub fn new(title: String, content: String, identifier: String) -> Self {
        Self {
            title,
            content,
            scroll_offset: 0,
            id: identifier,
            market_data: None,
            last_fetch: None,
            is_loading: false,
            error: None,
        }
    }

    pub fn should_refresh(&self) -> bool {
        match self.last_fetch {
            None => true,
            Some(last) => last.elapsed() >= Duration::from_secs(2),
        }
    }

    pub async fn fetch_market_data(&mut self) {
        self.is_loading = true;
        self.error = None;

        match get_market_from_slug(&self.id).await {
            Ok(data) => {
                self.market_data = Some(data);
                self.last_fetch = Some(Instant::now());
            }
            Err(e) => {
                self.error = Some(format!("{}", e));
            }
        }

        self.is_loading = false;
    }


}

impl Page for DetailPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, _state: &SharedState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title_block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(title_block, chunks[0]);

        let content_block = Block::default()
            .title("Details")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let display_content = if self.is_loading && self.market_data.is_none() {
            "Loading...".to_string()
        } else if let Some(ref error) = self.error {
            format!("Error: {}", error)
        } else if let Some(ref data) = self.market_data {

            format!(
                "{}\n\nMarket Data\n\nDescription: {}\nActive: {}\nLiquidity: {}\nVolume: {}\n24hr|1wk|1mo|1yr vol : {}|{}|{}|{}\nBid/Ask: {}/{}",
                    self.content,
                    data.description,
                    data.active,
                    data.liquidity,
                    data.volume,
                    data.volume24hr,
                    data.volume1wk,
                    data.volume1mo,
                    data.volume1yr,
                    data.best_ask,
                    data.best_bid
            )
        } else {
            format!("{}\n\nSlug: {}", self.content, self.id)
        };

        let content_paragraph = Paragraph::new(display_content)
            .block(content_block)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));
        let status = if self.is_loading { " (refreshing...)" } else { "" };

        frame.render_widget(content_paragraph, chunks[1]);

        let help_text = Line::from(vec![
            Span::styled("Esc/Backspace", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Go Back  "),
            Span::styled("↑/↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Scroll  "),
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
            Span::styled(status, Style::default().fg(Color::DarkGray)),
        ]);

        let help_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        frame.render_widget(
            Paragraph::new(help_text).block(help_block),
            chunks[2],
        );
    }

    fn handle_input(&mut self, key: KeyEvent, _state: &SharedState) -> PageAction {
        match key.code {
            KeyCode::Char('q') => PageAction::Quit,
            KeyCode::Esc | KeyCode::Backspace => PageAction::GoBack,
            KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                PageAction::None
            }
            KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                PageAction::None
            }
            KeyCode::Char('y') => {buy_yes(self.market_data.as_ref().clone().unwrap().clob_token_ids.clone(), "Yes"); PageAction::None}
            _ => PageAction::None,
        }
    }


}
