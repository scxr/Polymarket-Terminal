use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

use crate::data::state::SharedState;
use super::app::App;
use super::pages::{Page, PageAction, PageType};

pub fn run(state: SharedState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            match app.current_page {
                PageType::Dashboard => {
                    app.dashboard.render(frame, area, &state);
                }
                PageType::Detail => {
                    if let Some(ref mut detail) = app.detail_page {
                        detail.render(frame, area, &state);
                    }
                }
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = match app.current_page {
                    PageType::Dashboard => app.dashboard.handle_input(key, &state),
                    PageType::Detail => {
                        if let Some(ref mut detail) = app.detail_page {
                            detail.handle_input(key, &state)
                        } else {
                            PageAction::None
                        }
                    }
                };

                match action {
                    PageAction::None => {}
                    PageAction::Quit => break,
                    PageAction::GoBack => app.go_back(),
                    PageAction::NavigateToDetail { title, content } => {
                        app.navigate_to_detail(title, content);
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}