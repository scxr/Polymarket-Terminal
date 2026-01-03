use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::data::state::{SharedState, MarketData};
use super::{Page, PageAction};

#[derive(Clone, Copy, PartialEq)]
pub enum SelectedBox {
    TopMarkets,
    GeneralInfo,
    TopTraders,
    NewMarkets,
}

impl SelectedBox {
    fn left(&self) -> Self {
        match self {
            SelectedBox::TopMarkets => SelectedBox::NewMarkets,
            SelectedBox::GeneralInfo => SelectedBox::TopMarkets,
            SelectedBox::TopTraders => SelectedBox::GeneralInfo,
            SelectedBox::NewMarkets => SelectedBox::TopTraders,
        }
    }

    fn right(&self) -> Self {
        match self {
            SelectedBox::TopMarkets => SelectedBox::GeneralInfo,
            SelectedBox::GeneralInfo => SelectedBox::TopTraders,
            SelectedBox::TopTraders => SelectedBox::NewMarkets,
            SelectedBox::NewMarkets => SelectedBox::TopMarkets,
        }
    }

    fn up(&self) -> Self {
        match self {
            SelectedBox::TopMarkets => SelectedBox::TopTraders,
            SelectedBox::GeneralInfo => SelectedBox::NewMarkets,
            SelectedBox::TopTraders => SelectedBox::TopMarkets,
            SelectedBox::NewMarkets => SelectedBox::GeneralInfo,
        }
    }

    fn down(&self) -> Self {
        self.up()
    }
}

#[derive(Clone)]
pub struct FrameData {
    pub top_markets: Vec<MarketData>,
    pub top_traders: Vec<(String, f64)>,
    pub new_markets: Vec<(String, String)>,
    pub markets_updated_at: String,
    pub time_running: u64,
    pub total_markets: usize,
    pub total_trades: u64,
    pub total_volume: f64,
}

impl Default for FrameData {
    fn default() -> Self {
        Self {
            top_markets: vec![],
            top_traders: vec![],
            new_markets: vec![],
            markets_updated_at: "unknown".to_string(),
            time_running: 0,
            total_markets: 0,
            total_trades: 0,
            total_volume: 0.0,
        }
    }
}

pub struct DashboardPage {
    pub selected: SelectedBox,
    pub top_markets_index: usize,
    pub top_traders_index: usize,
    pub new_markets_index: usize,
    pub cached_frame_data: Option<FrameData>,
}

impl DashboardPage {
    pub fn new() -> Self {
        Self {
            selected: SelectedBox::TopMarkets,
            top_markets_index: 0,
            top_traders_index: 0,
            new_markets_index: 0,
            cached_frame_data: None,
        }
    }

    fn prepare_frame_data(&mut self, state: &SharedState) -> FrameData {
        match state.try_lock() {
            Ok(mut app_state) => {
                let top_markets = app_state.get_top_markets();
                let traders = app_state.get_top_traders();
                let new_markets = app_state.new_markets().clone();
                let markets_updated_at = app_state.last_updated_markets();
                let general_data = app_state.general_stats();

                let data = FrameData {
                    top_markets: top_markets.0,
                    top_traders: traders,
                    new_markets,
                    markets_updated_at,
                    time_running: general_data.1,
                    total_markets: general_data.0,
                    total_trades: general_data.2,
                    total_volume: general_data.3,
                };
                self.cached_frame_data = Some(data.clone());
                data
            }
            Err(_) => self.cached_frame_data.clone().unwrap_or_default(),
        }
    }

    fn get_selected_item_info(&self, frame_data: &FrameData) -> Option<(String, String)> {
        match self.selected {
            SelectedBox::TopMarkets => {
                frame_data.top_markets.get(self.top_markets_index).map(|m| {
                    (
                        format!("Market: {}", m.name),
                        format!(
                            "Name: {}\nVolume: {}\n\n[More details will go here]",
                            m.name,
                            format_volume(m.volume)
                        ),
                    )
                })
            }
            SelectedBox::TopTraders => {
                frame_data.top_traders.get(self.top_traders_index).map(|(addr, vol)| {
                    (
                        format!("Trader: {}", format_address(addr)),
                        format!(
                            "Address: {}\nVolume: {}\n\n[More details will go here]",
                            addr,
                            format_volume(*vol)
                        ),
                    )
                })
            }
            SelectedBox::NewMarkets => {
                frame_data.new_markets.get(self.new_markets_index).map(|(name, vol)| {
                    (
                        format!("New Market: {}", name),
                        format!(
                            "Name: {}\nVolume: {}\n\n[More details will go here]",
                            name, vol
                        ),
                    )
                })
            }
            SelectedBox::GeneralInfo => None,
        }
    }
}

impl Page for DashboardPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &SharedState) {
        let frame_data = self.prepare_frame_data(state);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let selected_border_style = Style::default().fg(Color::Yellow);
        let normal_border_style = Style::default();


        let top_markets_items: Vec<Line> = frame_data
            .top_markets
            .iter()
            .enumerate()
            .map(|(rank, m)| {
                let text = format!("{}. {} - {}", rank + 1, m.name, format_volume(m.volume));
                if self.selected == SelectedBox::TopMarkets && rank == self.top_markets_index {
                    Line::from(Span::styled(
                        text,
                        Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(text)
                }
            })
            .collect();

        let top_markets_block = Block::default()
            .title("Top Markets [↑/↓ select, Enter open, Tab switch]")
            .borders(Borders::ALL)
            .border_style(if self.selected == SelectedBox::TopMarkets {
                selected_border_style
            } else {
                normal_border_style
            });

        frame.render_widget(
            Paragraph::new(top_markets_items).block(top_markets_block),
            top_chunks[0],
        );

        let general_info_text = format!(
            "Running for: {} seconds\nTotal trades tracked: {}\nTotal markets discovered: {}\nTotal volume: ${}",
            frame_data.time_running,
            frame_data.total_trades,
            frame_data.total_markets,
            format_volume(frame_data.total_volume),
        );

        let general_info_block = Block::default()
            .title("General Info")
            .borders(Borders::ALL)
            .border_style(if self.selected == SelectedBox::GeneralInfo {
                selected_border_style
            } else {
                normal_border_style
            });

        frame.render_widget(
            Paragraph::new(general_info_text).block(general_info_block),
            top_chunks[1],
        );

        let top_traders_items: Vec<Line> = frame_data
            .top_traders
            .iter()
            .enumerate()
            .map(|(rank, (addr, vol))| {
                let text = format!("{}. {} - {}", rank + 1, format_address(addr), format_volume(*vol));
                if self.selected == SelectedBox::TopTraders && rank == self.top_traders_index {
                    Line::from(Span::styled(
                        text,
                        Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(text)
                }
            })
            .collect();

        let traders_block = Block::default()
            .title("Top Traders")
            .borders(Borders::ALL)
            .border_style(if self.selected == SelectedBox::TopTraders {
                selected_border_style
            } else {
                normal_border_style
            });

        frame.render_widget(
            Paragraph::new(top_traders_items).block(traders_block),
            bottom_chunks[0],
        );
        let new_markets_items: Vec<Line> = frame_data
            .new_markets
            .iter()
            .enumerate()
            .map(|(idx, (name, volume))| {
                let text = format!("{} - {}", name, volume);
                if self.selected == SelectedBox::NewMarkets && idx == self.new_markets_index {
                    Line::from(Span::styled(
                        text,
                        Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(text)
                }
            })
            .collect();

        let new_markets_block = Block::default()
            .title(format!(
                "New Markets - Last updated {} ago",
                frame_data.markets_updated_at
            ))
            .borders(Borders::ALL)
            .border_style(if self.selected == SelectedBox::NewMarkets {
                selected_border_style
            } else {
                normal_border_style
            });

        frame.render_widget(
            Paragraph::new(new_markets_items).block(new_markets_block),
            bottom_chunks[1],
        );
    }

    fn handle_input(&mut self, key: KeyEvent, state: &SharedState) -> PageAction {
        let frame_data = self.prepare_frame_data(state);

        match key.code {
            KeyCode::Char('q') => PageAction::Quit,
            KeyCode::Esc => PageAction::Quit,
            KeyCode::Left => {
                self.selected = self.selected.left();
                PageAction::None
            }
            KeyCode::Right => {
                self.selected = self.selected.right();
                PageAction::None
            }
            KeyCode::Up => {
                match self.selected {
                    SelectedBox::TopMarkets => {
                        if self.top_markets_index > 0 {
                            self.top_markets_index -= 1;
                        }
                    }
                    SelectedBox::TopTraders => {
                        if self.top_traders_index > 0 {
                            self.top_traders_index -= 1;
                        }
                    }
                    SelectedBox::NewMarkets => {
                        if self.new_markets_index > 0 {
                            self.new_markets_index -= 1;
                        }
                    }
                    SelectedBox::GeneralInfo => {
                        self.selected = self.selected.up();
                    }
                }
                PageAction::None
            }
            KeyCode::Down => {
                match self.selected {
                    SelectedBox::TopMarkets => {
                        if self.top_markets_index < frame_data.top_markets.len().saturating_sub(1) {
                            self.top_markets_index += 1;
                        }
                    }
                    SelectedBox::TopTraders => {
                        if self.top_traders_index < frame_data.top_traders.len().saturating_sub(1) {
                            self.top_traders_index += 1;
                        }
                    }
                    SelectedBox::NewMarkets => {
                        if self.new_markets_index < frame_data.new_markets.len().saturating_sub(1) {
                            self.new_markets_index += 1;
                        }
                    }
                    SelectedBox::GeneralInfo => {
                        self.selected = self.selected.down();
                    }
                }
                PageAction::None
            }
            KeyCode::Enter => {
                if let Some((title, content)) = self.get_selected_item_info(&frame_data) {
                    PageAction::NavigateToDetail { title, content }
                } else {
                    PageAction::None
                }
            }
            KeyCode::Tab => {
                self.selected = match self.selected {
                    SelectedBox::TopMarkets => SelectedBox::GeneralInfo,
                    SelectedBox::GeneralInfo => SelectedBox::TopTraders,
                    SelectedBox::TopTraders => SelectedBox::NewMarkets,
                    SelectedBox::NewMarkets => SelectedBox::TopMarkets,
                };
                PageAction::None
            }
            _ => PageAction::None,
        }
    }
}

fn format_volume(volume: f64) -> String {
    if volume >= 1_000_000_000.0 {
        format!("{:.2}B", volume / 1_000_000_000.0)
    } else if volume >= 1_000_000.0 {
        format!("{:.2}M", volume / 1_000_000.0)
    } else if volume >= 1_000.0 {
        format!("{:.2}K", volume / 1_000.0)
    } else {
        format!("{:.2}", volume)
    }
}

fn format_address(address: &str) -> String {
    if address.len() <= 10 {
        address.to_string()
    } else {
        format!("{}...{}", &address[..6], &address[address.len() - 4..])
    }
}