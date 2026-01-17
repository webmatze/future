use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Gauge},
};

use crate::data::system_stats::{format_bytes, SystemStats};
use crate::ui::theme::*;

pub struct MemoryGaugeState {
    pub used: u64,
    pub total: u64,
    pub percentage: f64,
}

impl MemoryGaugeState {
    pub fn new() -> Self {
        Self {
            used: 0,
            total: 0,
            percentage: 0.0,
        }
    }

    pub fn update(&mut self, stats: &SystemStats) {
        self.used = stats.memory_used;
        self.total = stats.memory_total;
        self.percentage = stats.memory_percentage();
    }
}

impl Default for MemoryGaugeState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_memory_gauge(frame: &mut Frame, state: &MemoryGaugeState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" MEMORY ")
        .title_style(Style::default().fg(NEON_MAGENTA).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Main gauge
    let gauge = Gauge::default()
        .ratio(state.percentage / 100.0)
        .gauge_style(Style::default().fg(gauge_color(state.percentage)).bg(DARK_BG))
        .label(format!(
            "{} / {} ({:.1}%)",
            format_bytes(state.used),
            format_bytes(state.total),
            state.percentage
        ));

    frame.render_widget(gauge, chunks[0]);

    // Info line
    if chunks[1].height > 0 {
        let info = Line::from(vec![
            Span::styled("Used: ", Style::default().fg(TEXT_DIM)),
            Span::styled(format_bytes(state.used), Style::default().fg(NEON_MAGENTA)),
            Span::styled("  Free: ", Style::default().fg(TEXT_DIM)),
            Span::styled(
                format_bytes(state.total.saturating_sub(state.used)),
                Style::default().fg(NEON_GREEN),
            ),
        ]);

        frame.render_widget(
            ratatui::widgets::Paragraph::new(info),
            chunks[1],
        );
    }
}
