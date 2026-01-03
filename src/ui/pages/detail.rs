use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::data::state::SharedState;
use super::{Page, PageAction};

pub struct DetailPage {
    pub title: String,
    pub content: String,
    pub scroll_offset: u16,
}

impl DetailPage {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            scroll_offset: 0,
        }
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

        let content_paragraph = Paragraph::new(self.content.clone())
            .block(content_block)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));

        frame.render_widget(content_paragraph, chunks[1]);

        let help_text = Line::from(vec![
            Span::styled("Esc/Backspace", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Go Back  "),
            Span::styled("↑/↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Scroll  "),
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
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
            _ => PageAction::None,
        }
    }
}