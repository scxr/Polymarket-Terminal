use crate::data::new_markets::{self, get_new_markets};
use crate::data::state::{AppState, SharedState};
use crate::data::state::MarketData;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::widgets::List;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
};

use std::fmt::format;
use std::io;
use std::time::Duration;

const WINDOW_SIZE: f64 = 120.0;
const TOP_MARKETS_COUNT: usize = 10;

pub fn run(state: SharedState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut tick = 0;
    loop {
        let frame_data = prepare_frame_data(&state, tick);
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
                .map(|(rank, m)| Line::from(format!("{}. {} - {}", rank + 1, m.name, format_volume(m.volume)))) // use actual field names
                .collect();

            let items_traders: Vec<Line> = frame_data.top_traders
                .iter()
                .enumerate()
                .map(|(rank, (addr, vol))| Line::from(format!("{}. {} - {}", rank + 1, format_address(addr), format_volume(*vol)))) // use actual field names
                .collect();

            let list = Paragraph::new(items)
                .block(Block::default().title("Top Markets").borders(Borders::ALL));

            let trader_list = Paragraph::new(items_traders)
                .block(Block::default().title("Top Traders").borders(Borders::ALL));

            let new_markets_items: Vec<Line> = frame_data.new_markets
                .iter()
                .map(|(name, volume)| Line::from(format!("{} - {}", name, volume)))
                .collect();

            frame.render_widget(list, horchunkstop[0]);
            frame.render_widget(Paragraph::new(format!("Running for: {} seconds\nTotal trades tracked: {}\nTotal markets discovered: {}\nTotal volume: ${}", frame_data.time_running, frame_data.total_trades, frame_data.total_markets, frame_data.total_volume)).block(
                Block::default().title("General Info").borders(Borders::ALL)
            ), horchunkstop[1]);


            frame.render_widget(trader_list, horchunks_btm[0]);
            frame.render_widget(Paragraph::new(new_markets_items).block(Block::default().title(format!("New Markets - Last updated {} ago", frame_data.markets_updated_at)).borders(Borders::ALL)), horchunks_btm[1]);

            tick += 1;
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

struct FrameData {
    top_markets: Vec<MarketData>,
    volume: f64,
    top_traders: Vec<(String, f64)>,
    new_markets: Vec<(String, String)>,
    markets_updated_at: String,
    time_running: u64,
    total_markets: usize,
    total_trades: u64,
    total_volume: f64,

}

fn prepare_frame_data(state: &SharedState, tick: u64) -> FrameData {
    let mut app_state = state.lock().unwrap();
    let top_markets = app_state.get_top_markets();
    let traders = app_state.get_top_traders();
    let new_markets = app_state.new_markets().clone();
    let markets_updated_at = app_state.last_updated_markets();
    let general_data = app_state.general_stats();
    FrameData {
        top_markets: top_markets.0,
        volume: top_markets.1,
        top_traders: traders,
        new_markets,
        markets_updated_at,
        time_running: general_data.1,
        total_markets: general_data.0,
        total_trades: general_data.2,
        total_volume: general_data.3,

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