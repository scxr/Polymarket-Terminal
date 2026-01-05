use std::env;
use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::Stylize;
use alloy::{primitives::address, providers::ProviderBuilder, sol};
use crate::actions::approvals::approval_process;
use crate::actions::wallet_info::get_wallet_full;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},

    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::data::state::SharedState;
use dotenv::dotenv;

use super::{Page, PageAction};
pub struct WalletPage {
    pub title: String,
    pub needs_wallet_update: bool,
    pub pending_approval: bool,
    pub approval_text: String,
}

impl WalletPage {
    pub fn new(title: String) -> Self {
        Self {
            title: title,
            needs_wallet_update: true,
            pending_approval: false,
            approval_text: "Approval Process: Not running".to_string(),

        }
    }

    pub fn needs_wallet_update(&self) -> bool {
        self.needs_wallet_update
    }

    pub async fn fetch_wallet_info(&mut self) {
        dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");
        let wallet_details =get_wallet_full(private_key.as_str()).await;
        match wallet_details {
            Ok(wallet_details) => {
                self.title = format!("Wallet  info fetched\nAddress: {}\nUSDCE Balance: {}\nPOL Balance: {}\n\nUser is approved? {}\n\n{} ", wallet_details.0, wallet_details.1, wallet_details.2, wallet_details.3, self.approval_text).to_string();
            }
            Err(e) => {
                self.title = String::from("Error parsing private key");
            }
        }
        self.needs_wallet_update = false;
    }

    pub fn needs_approval(&self) -> bool {
        self.pending_approval
    }


    pub async fn run_approval(&mut self) {
        dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");

        self.title = "Running approval process...".to_string();

        match approval_process(&private_key).await {
            Ok(result) => {
                if result.success {
                    self.approval_text = "Successfully approved".to_string();
                    let approvals_str: Vec<String> = result.approvals
                        .iter()
                        .enumerate()
                        .filter_map(|(i, opt)| {
                            opt.as_ref().map(|hash| format!("Spender {}: {}", i + 1, hash))
                        })
                        .collect();

                    if approvals_str.is_empty() {
                        self.approval_text = "Approval process complete!\nAll spenders were already approved.".to_string();
                    } else {
                        self.title = format!(
                            "Approval process complete!\n\nNew approvals:\n{}",
                            approvals_str.join("\n")
                        );
                    }
                } else {
                    self.approval_text = format!(
                        "Approval failed: {}",
                        result.error.unwrap_or_else(|| "Unknown error".to_string())
                    );
                }
            }
            Err(e) => {
                self.title = format!("Approval process error: {:?}", e);
            }
        }

        self.pending_approval = false;
        self.needs_wallet_update = true;
    }
}

impl Page for WalletPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &SharedState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
            ])
            .split(area);
        let content_block = Block::default().borders(Borders::ALL);

        let contented_paragraph = Paragraph::new(self.title.clone()).wrap(Wrap { trim: true })
            .block(content_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(contented_paragraph, chunks[0]);
    }

    fn handle_input(&mut self, key: KeyEvent, state: &SharedState) -> PageAction {
        match key.code {
            KeyCode::Char('q') => PageAction::Quit,
            KeyCode::Esc | KeyCode::Backspace => PageAction::GoBack,
            KeyCode::Char('a') => {
                self.pending_approval = true;
                self.title="Approving...".to_string();
                PageAction::None}
            _ => PageAction::None,
        }
    }
}