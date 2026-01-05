use std::env;
use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::Stylize;
use dotenv::dotenv;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
};
use crate::data::get_market::get_market_from_slug;
use crate::data::state::SharedState;
use crate::data::types::MarketSpecificDetails;
use crate::actions::buy::buy_yes;
use super::{Page, PageAction};

#[derive(PartialEq, Clone)]
pub enum InputMode {
    Normal,
    BuyYes,
    BuyNo,
}

pub struct DetailPage {
    pub title: String,
    pub content: String,
    pub scroll_offset: u16,
    pub id: String,
    pub market_data: Option<MarketSpecificDetails>,
    pub last_fetch: Option<Instant>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub buy_yes: bool,
    pub buy_no: bool,
    private_key: String,
    pub buy_resp: String,
    pub input_mode: InputMode,
    pub input_buffer: String,
}

impl DetailPage {
    pub fn new(title: String, content: String, identifier: String) -> Self {
        dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string());
        Self {
            title,
            content,
            scroll_offset: 0,
            id: identifier,
            market_data: None,
            last_fetch: None,
            is_loading: false,
            error: None,
            private_key,
            buy_yes: false,
            buy_no: false,
            buy_resp: "".to_string(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
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

    pub fn should_buy_yes(&mut self) -> bool {
        self.buy_yes
    }

    pub fn should_buy_no(&mut self) -> bool {
        self.buy_no
    }

    pub fn get_buy_amount(&self) -> Option<f64> {
        self.input_buffer.parse().ok()
    }

    pub async fn buy(&mut self, yes: bool, amount: f64) {
        let side = if yes { "Yes" } else { "No" };
        self.buy_resp = "Processing...".to_string();

        let resp = buy_yes(
            &self.private_key,
            self.market_data.as_ref().unwrap().clob_token_ids.clone(),
            side,
            amount.to_string()
        ).await;

        match resp {
            Ok(response) => {
                let error_msg = response.error_msg.unwrap_or_default();
                if !error_msg.is_empty() {
                    self.buy_resp = format!("There was an error buying: {}", error_msg);
                } else {
                    self.buy_resp = format!(
                        "Order Status: {}\nYou spent: ${} and received {} {} shares",
                        response.status, response.making_amount, response.taking_amount, side
                    );
                }
            }
            Err(e) => {
                self.buy_resp = format!("Buy error: {}", e);
            }
        }

        self.buy_yes = false;
        self.buy_no = false;
    }

    fn render_input_popup(&self, frame: &mut Frame, area: Rect) {
        let popup_width = 40;
        let popup_height = 5;

        let popup_area = Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width.min(area.width),
            height: popup_height.min(area.height),
        };

        frame.render_widget(Clear, popup_area);

        let side = match self.input_mode {
            InputMode::BuyYes => "YES",
            InputMode::BuyNo => "NO",
            InputMode::Normal => "",
        };

        let block = Block::default()
            .title(format!(" Buy {} - Enter Amount ", side))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let input_text = Line::from(vec![
            Span::raw("$ "),
            Span::styled(&self.input_buffer, Style::default().fg(Color::White)),
            Span::styled("│", Style::default().fg(Color::Gray)), // cursor
        ]);

        let help_line = Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" Confirm  "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(" Cancel"),
        ]);

        let content = vec![input_text, Line::raw(""), help_line];

        let paragraph = Paragraph::new(content)
            .block(block)
            .style(Style::default().bg(Color::Black));

        frame.render_widget(paragraph, popup_area);
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
                "{}\n\nMarket Data\n\nDescription: {}\nActive: {}\nLiquidity: {}\nVolume: {}\n24hr|1wk|1mo|1yr vol : {}|{}|{}|{}\nBid/Ask: {}/{}\n\n\n{}",
                self.content,
                data.description,
                data.active,
                data.liquidity,
                data.volume,
                data.volume24hr.unwrap_or(0.0),
                data.volume1wk.unwrap_or(0.0),
                data.volume1mo.unwrap_or(0.0),
                data.volume1yr.unwrap_or(0.0),
                data.best_ask,
                data.best_bid,
                self.buy_resp
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
            Span::styled("y/n", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Buy Yes/No  "),
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

        if self.input_mode != InputMode::Normal {
            self.render_input_popup(frame, area);
        }
    }

    fn handle_input(&mut self, key: KeyEvent, _state: &SharedState) -> PageAction {
        if self.input_mode != InputMode::Normal {
            match key.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                    PageAction::None
                }
                KeyCode::Enter => {
                    if !self.input_buffer.is_empty() {
                        if self.input_buffer.parse::<f64>().is_ok() {
                            match self.input_mode.clone() {
                                InputMode::BuyYes => self.buy_yes = true,
                                InputMode::BuyNo => self.buy_no = true,
                                _ => {}
                            }
                        } else {
                            self.input_buffer.clear();
                        }
                    }
                    self.input_mode = InputMode::Normal;
                    PageAction::None
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    PageAction::None
                }
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() || (c == '.' && !self.input_buffer.contains('.')) {
                        self.input_buffer.push(c);
                    }
                    PageAction::None
                }
                _ => PageAction::None,
            }
        } else {
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
                KeyCode::Char('y') => {
                    self.input_mode = InputMode::BuyYes;
                    self.input_buffer.clear();
                    PageAction::None
                }
                KeyCode::Char('n') => {
                    self.input_mode = InputMode::BuyNo;
                    self.input_buffer.clear();
                    PageAction::None
                }
                _ => PageAction::None,
            }
        }
    }
}