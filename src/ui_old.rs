mod runner;

use crate::data::state::{ SharedState};
use crate::data::state::MarketData;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};


use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use std::io;
use std::time::Duration;


#[derive(Clone, Copy, PartialEq)]
enum SelectedBox {
    TopMarkets,
    GeneralInfo,
    TopTraders,
    NewMarkets
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

struct SelectionState {
    selected: SelectedBox,
    top_markets_index: usize,
    top_traders_index: usize,
    new_markets_index: usize,
    selected_item: Option<String>,
    cached_frame_data: Option<FrameData>,
}

impl SelectionState {
    fn new() -> Self {
        Self {
            selected: SelectedBox::TopMarkets,
            top_traders_index: 0,
            top_markets_index: 0,
            new_markets_index: 0,
            selected_item: None,
            cached_frame_data: None,
        }
    }
}

pub fn run(state: SharedState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);


    let mut terminal = Terminal::new(backend)?;
    let mut selection = SelectionState::new();
    loop {
        let frame_data =  prepare_frame_data(&state, &mut selection.cached_frame_data);
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(frame.area());
            let horchunkstop = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(chunks[0]);

            let horchunks_btm = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(chunks[1]);


            let items: Vec<Line> = frame_data.top_markets
                .iter()
                .enumerate()
                .map(|(rank, m)| {
                    let text = format!("{}. {} - {}", rank + 1, m.name, format_volume(m.volume));
                    if selection.selected == SelectedBox::TopMarkets && rank == selection.top_markets_index {
                        Line::from(Span::styled(text, Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD)))
                    } else {
                        Line::from(text)
                    }
                })
                .collect();

            let items_traders: Vec<Line> = frame_data.top_traders
                .iter()
                .enumerate()
                .map(|(rank, (addr, vol))| {
                    let text = format!("{}. {} - {}", rank + 1, format_address(addr), format_volume(*vol));
                    if selection.selected == SelectedBox::TopTraders && rank == selection.top_traders_index {
                        Line::from(Span::styled(text, Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD)))
                    } else {
                        Line::from(text)
                    }
                })
                .collect();

            let list = Paragraph::new(items)
                .block(Block::default().title("Top Markets").borders(Borders::ALL));

            let trader_list = Paragraph::new(items_traders)
                .block(Block::default().title("Top Traders").borders(Borders::ALL));

            let new_markets_items: Vec<Line> = frame_data.new_markets
                .iter()
                .enumerate()
                .map(|(idx, (name, volume))| {
                    let text = format!("{} - {}", name, volume);
                    if selection.selected == SelectedBox::NewMarkets && idx == selection.new_markets_index {
                        Line::from(Span::styled(text, Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD)))
                    } else {
                        Line::from(text)
                    }
                })
                .collect();

            let selected_border_style = Style::default().fg(Color::Yellow);
            let normal_border_style = Style::default();

            let top_markets_block = Block::default()
                .title("Top Markets [←/→ to switch, ↑/↓ to select, Enter to confirm]")
                .borders(Borders::ALL)
                .border_style(if selection.selected == SelectedBox::TopMarkets { selected_border_style } else { normal_border_style });

            let general_info_block = Block::default()
                .title("General Info")
                .borders(Borders::ALL)
                .border_style(if selection.selected == SelectedBox::GeneralInfo { selected_border_style } else { normal_border_style });

            let trader_block = Block::default()
                .title("Top Traders")
                .borders(Borders::ALL)
                .border_style(if selection.selected == SelectedBox::TopTraders { selected_border_style } else { normal_border_style });

            let new_markets_block = Block::default()
                .title(format!("New Markets - Last updated {} ago", frame_data.markets_updated_at))
                .borders(Borders::ALL)
                .border_style(if selection.selected == SelectedBox::NewMarkets { selected_border_style } else { normal_border_style });


            let general_info_text = format!(
                "Running for: {} seconds\nTotal trades tracked: {}\nTotal markets discovered: {}\nTotal volume: ${}\n\n{}",
                frame_data.time_running,
                frame_data.total_trades,
                frame_data.total_markets,
                frame_data.total_volume,
                if let Some(ref item) = selection.selected_item {
                    format!("Selected: {}", item)
                } else {
                    "No item selected".to_string()
                }
            );

            frame.render_widget(list, horchunkstop[0]);
            frame.render_widget(
                Paragraph::new(general_info_text).block(general_info_block),
                horchunkstop[1]
            );
            frame.render_widget(trader_list, horchunks_btm[0]);
            frame.render_widget(
                Paragraph::new(new_markets_items).block(new_markets_block),
                horchunks_btm[1]
            );
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        selection.selected = selection.selected.left();
                    }
                    KeyCode::Right => {
                        selection.selected = selection.selected.right();
                    }
                    KeyCode::Up => {
                        match selection.selected {
                            SelectedBox::TopMarkets => {
                                if selection.top_markets_index > 0 {
                                    selection.top_markets_index -= 1;
                                }
                            }
                            SelectedBox::TopTraders => {
                                if selection.top_traders_index > 0 {
                                    selection.top_traders_index -= 1;
                                }
                            }
                            SelectedBox::NewMarkets => {
                                if selection.new_markets_index > 0 {
                                    selection.new_markets_index -= 1;
                                }
                            }
                            SelectedBox::GeneralInfo => {
                                selection.selected = selection.selected.up();
                            }
                        }
                    }
                    KeyCode::Down => {
                        let frame_data = prepare_frame_data(&state, &mut selection.cached_frame_data);
                        match selection.selected {
                            SelectedBox::TopMarkets => {
                                if selection.top_markets_index < frame_data.top_markets.len().saturating_sub(1) {
                                    selection.top_markets_index += 1;
                                }
                            }
                            SelectedBox::TopTraders => {
                                if selection.top_traders_index < frame_data.top_traders.len().saturating_sub(1) {
                                    selection.top_traders_index += 1;
                                }
                            }
                            SelectedBox::NewMarkets => {
                                if selection.new_markets_index < frame_data.new_markets.len().saturating_sub(1) {
                                    selection.new_markets_index += 1;
                                }
                            }
                            SelectedBox::GeneralInfo => {
                                selection.selected = selection.selected.down();
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let frame_data = prepare_frame_data(&state, &mut selection.cached_frame_data);
                        selection.selected_item = match selection.selected {
                            SelectedBox::TopMarkets => {
                                frame_data.top_markets.get(selection.top_markets_index)
                                    .map(|m| format!("Market: {} (Vol: {})", m.name, format_volume(m.volume)))
                            }
                            SelectedBox::TopTraders => {
                                frame_data.top_traders.get(selection.top_traders_index)
                                    .map(|(addr, vol)| format!("Trader: {} (Vol: {})", format_address(addr), format_volume(*vol)))
                            }
                            SelectedBox::NewMarkets => {
                                frame_data.new_markets.get(selection.new_markets_index)
                                    .map(|(name, vol)| format!("New Market: {} (Vol: {})", name, vol))
                            }
                            SelectedBox::GeneralInfo => None,
                        };
                    }
                    KeyCode::Tab => {
                        selection.selected = match selection.selected {
                            SelectedBox::TopMarkets => SelectedBox::GeneralInfo,
                            SelectedBox::GeneralInfo => SelectedBox::TopTraders,
                            SelectedBox::TopTraders => SelectedBox::NewMarkets,
                            SelectedBox::NewMarkets => SelectedBox::TopMarkets,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
#[derive(Clone)]
struct FrameData {
    top_markets: Vec<MarketData>,
    top_traders: Vec<(String, f64)>,
    new_markets: Vec<(String, String)>,
    markets_updated_at: String,
    time_running: u64,
    total_markets: usize,
    total_trades: u64,
    total_volume: f64,

}

fn prepare_frame_data(state: &SharedState, cache: &mut Option<FrameData>) -> FrameData {
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
            *cache = Some(data.clone());
            data
        }
        Err(_) => {
            cache.clone().unwrap_or_else(|| FrameData {
                top_markets: vec![],
                top_traders: vec![],
                new_markets: vec![],
                markets_updated_at: "unknown".to_string(),
                time_running: 0,
                total_markets: 0,
                total_trades: 0,
                total_volume: 0.0,
            })
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