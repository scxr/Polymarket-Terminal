use std::sync::{Arc};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub type SharedState = Arc<Mutex<AppState>>;

#[derive(Clone)]
pub struct MarketData {
    pub name: String,
    pub volume: f64,
    pub identifier: String,
}
pub struct AppState {
    pub traders: Vec<(String, f64)>,
    pub top_markets: HashMap<String, MarketData>,
    pub closing_markets: Vec<String>,
    pub tracked_markets: u64,
    pub tracked_traders: HashMap<String, f64>,
    pub tick: u64,
    pub new_markets: Vec<(String, String)>,
    pub markets_updated: u64,
    started_at: SystemTime,
}
const DISPLAY_COUNT: usize = 50;

impl AppState {
    pub fn new() -> Self {
        Self {
            new_markets: vec!(),
            traders: vec!(),
            top_markets: HashMap::new(),
            closing_markets: vec!(),
            tracked_traders: HashMap::new(),
            tracked_markets: 0,
            tick: 0,
            markets_updated: 0,
            started_at: SystemTime::now(),

        }
    }

    pub fn add_trade(&mut self, title: String, trade_size: f64, trader_address: String, id: String ) {
        self.tick += 1;
        self.tracked_markets += 1;
        self.tracked_traders.entry(trader_address).and_modify(|v| *v += trade_size).or_insert(trade_size);
        self.top_markets.entry(title.clone()).and_modify(|v| v.volume += trade_size).or_insert(MarketData {name: title,volume: trade_size, identifier: id });
    }

    pub fn get_top_markets(&mut self) -> (Vec<MarketData>, f64) {
        let mut top_markets_vals = self.top_markets.values().cloned().collect::<Vec<_>>();
        top_markets_vals.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());
        (top_markets_vals.into_iter().take(DISPLAY_COUNT).collect(), self.tracked_markets as f64)
    }

    pub fn get_top_traders(&mut self) -> Vec<(String, f64)> {
        let mut traders_vec = self.tracked_traders.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>();
        traders_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        traders_vec.into_iter().take(DISPLAY_COUNT).collect()
    }

    pub fn set_new_markets(&mut self, markets: Vec<(String, String)>) {
        self.new_markets = markets;
        self.markets_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    pub fn general_stats(&self) -> (usize, u64, u64, f64) {
        let time_running = self.started_at.elapsed().unwrap().as_secs();
        let total_trades =self.tick;
        let total_markets = self.top_markets.len();
        let mut total_volume: f64  = 0.0;
        for x in self.top_markets.values() {
            total_volume += x.volume;
        }
        (
            total_markets,
            time_running,
            total_trades,
            total_volume
        )
    }


    pub fn last_updated_markets(&self) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if self.markets_updated == 0 {
            return String::from("never");
        }
        let diff = now.saturating_sub(self.markets_updated);
        format!("{} secs", diff)
    }

    pub fn new_markets(&self) -> &Vec<(String, String)> {
         &self.new_markets
    }

    pub fn increment_market_count(&mut self) {
        self.tracked_markets += 1;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}