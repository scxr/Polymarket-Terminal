mod data;
mod ui;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph}};
use std::sync::{Arc, Mutex};

use crate::data::state::{AppState, SharedState};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   
    let state: SharedState = Arc::new(Mutex::new(AppState::new()));
    let ws_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = data::ws::run(ws_state).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });
    ui::run(state);
    Ok(())
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    if c == 'q' {
                        break Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(outer_layout[0]);

    let inner_layout2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(outer_layout[1]);

    frame.render_widget(Paragraph::new("q1").block(Block::new().borders(Borders::ALL)), inner_layout[0]);
    frame.render_widget(Paragraph::new("q2").block(Block::new().borders(Borders::ALL)), inner_layout[1]);
    frame.render_widget(Paragraph::new("q3").block(Block::new().borders(Borders::ALL)), inner_layout2[0]);
    frame.render_widget(Paragraph::new("q4").block(Block::new().borders(Borders::ALL)), inner_layout2[1]);

}