use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
};

use crate::ui::theme::*;

pub struct CountdownState {
    pub initial_seconds: u64,
    pub remaining_seconds: u64,
    tick_counter: u64,
    flash_state: bool,
}

impl CountdownState {
    pub fn new(seconds: u64) -> Self {
        Self {
            initial_seconds: seconds,
            remaining_seconds: seconds,
            tick_counter: 0,
            flash_state: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;

        // Update flash state every 15 ticks
        if self.tick_counter % 15 == 0 {
            self.flash_state = !self.flash_state;
        }

        // Decrement every 60 ticks (1 second at 60fps)
        if self.tick_counter % 60 == 0 && self.remaining_seconds > 0 {
            self.remaining_seconds -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.remaining_seconds = self.initial_seconds;
        self.tick_counter = 0;
    }

    pub fn is_critical(&self) -> bool {
        self.remaining_seconds <= 10
    }

    pub fn is_warning(&self) -> bool {
        self.remaining_seconds <= 60
    }

    pub fn format_time(&self) -> String {
        let hours = self.remaining_seconds / 3600;
        let minutes = (self.remaining_seconds % 3600) / 60;
        let seconds = self.remaining_seconds % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}

impl Default for CountdownState {
    fn default() -> Self {
        Self::new(300)
    }
}

pub fn render_countdown(frame: &mut Frame, state: &CountdownState, area: Rect) {
    let color = if state.remaining_seconds == 0 {
        if state.flash_state { NEON_RED } else { DARK_BG }
    } else if state.is_critical() {
        if state.flash_state { NEON_RED } else { NEON_ORANGE }
    } else if state.is_warning() {
        NEON_ORANGE
    } else {
        NEON_CYAN
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" COUNTDOWN ")
        .title_style(Style::default().fg(color).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    let time_str = state.format_time();

    // Create large ASCII digits for dramatic effect
    let large_time = if inner.height >= 3 && inner.width >= time_str.len() as u16 * 4 {
        create_large_digits(&time_str)
    } else {
        vec![Line::from(Span::styled(
            time_str,
            Style::default().fg(color).bold(),
        ))]
    };

    let status_line = if state.remaining_seconds == 0 {
        Line::from(Span::styled(
            "█ EXPIRED █",
            Style::default().fg(NEON_RED).bold(),
        ))
    } else if state.is_critical() {
        Line::from(Span::styled(
            "⚠ CRITICAL",
            Style::default().fg(NEON_RED),
        ))
    } else if state.is_warning() {
        Line::from(Span::styled(
            "◆ WARNING",
            Style::default().fg(NEON_ORANGE),
        ))
    } else {
        Line::from(Span::styled(
            "◇ ACTIVE",
            Style::default().fg(NEON_GREEN),
        ))
    };

    let mut lines = large_time;
    lines.push(status_line);

    let countdown = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color));

    frame.render_widget(countdown, inner);
}

/// Create large ASCII art digits (simplified version)
fn create_large_digits(time: &str) -> Vec<Line<'static>> {
    // Simple block digits - each digit is 3 chars wide
    let digit_top: [&str; 11] = [
        "█▀█", "▀█ ", "▀▀█", "▀▀█", "█ █", "█▀▀", "█▀▀", "▀▀█", "█▀█", "█▀█", " ▄ ",
    ];
    let digit_mid: [&str; 11] = [
        "█ █", " █ ", "█▀▀", " ▀█", "▀▀█", "▀▀█", "█▀█", "  █", "█▀█", "▀▀█", "   ",
    ];
    let digit_bot: [&str; 11] = [
        "▀▀▀", "▀▀▀", "▀▀▀", "▀▀▀", "  ▀", "▀▀▀", "▀▀▀", "  ▀", "▀▀▀", "▀▀▀", " ▀ ",
    ];

    let mut top = String::new();
    let mut mid = String::new();
    let mut bot = String::new();

    for ch in time.chars() {
        let idx = match ch {
            '0'..='9' => ch as usize - '0' as usize,
            ':' => 10,
            _ => continue,
        };

        top.push_str(digit_top[idx]);
        top.push(' ');
        mid.push_str(digit_mid[idx]);
        mid.push(' ');
        bot.push_str(digit_bot[idx]);
        bot.push(' ');
    }

    vec![
        Line::from(top),
        Line::from(mid),
        Line::from(bot),
    ]
}
