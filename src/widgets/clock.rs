use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
};
use chrono::Local;

use crate::ui::theme::*;

pub struct ClockState {
    pub time_str: String,
    pub date_str: String,
    pub millis: String,
    tick_counter: u64,
}

impl ClockState {
    pub fn new() -> Self {
        let mut state = Self {
            time_str: String::new(),
            date_str: String::new(),
            millis: String::new(),
            tick_counter: 0,
        };
        state.update_time();
        state
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
        // Update time every tick for smooth millisecond display
        self.update_time();
    }

    fn update_time(&mut self) {
        let now = Local::now();
        self.time_str = now.format("%H:%M:%S").to_string();
        self.date_str = now.format("%Y-%m-%d").to_string();
        self.millis = now.format(".%3f").to_string();
    }
}

impl Default for ClockState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_clock(frame: &mut Frame, state: &ClockState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" SYSTEM TIME ")
        .title_style(Style::default().fg(NEON_CYAN).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    let blink = (state.tick_counter / 30) % 2 == 0;

    let time_line = Line::from(vec![
        Span::styled(&state.time_str, Style::default().fg(TEXT_HIGHLIGHT).bold()),
        Span::styled(
            &state.millis,
            Style::default().fg(if blink { NEON_CYAN } else { TEXT_DIM }),
        ),
    ]);

    let date_line = Line::from(vec![
        Span::styled(&state.date_str, Style::default().fg(TEXT_DIM)),
        Span::styled("  ", Style::default()),
        Span::styled("●", Style::default().fg(if blink { NEON_GREEN } else { TEXT_DIM })),
        Span::styled(" SYNC", Style::default().fg(TEXT_DIM)),
    ]);

    let clock_text = vec![time_line, date_line];

    let clock = Paragraph::new(clock_text).alignment(Alignment::Center);

    frame.render_widget(clock, inner);
}
