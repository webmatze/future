use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Gauge, Sparkline},
};
use std::collections::VecDeque;

use crate::data::system_stats::SystemStats;
use crate::ui::theme::*;

const HISTORY_SIZE: usize = 60;

pub struct CpuGaugeState {
    pub current: f64,
    pub history: VecDeque<u64>,
}

impl CpuGaugeState {
    pub fn new() -> Self {
        let mut history = VecDeque::with_capacity(HISTORY_SIZE);
        // Initialize with zeros
        for _ in 0..HISTORY_SIZE {
            history.push_back(0);
        }

        Self {
            current: 0.0,
            history,
        }
    }

    pub fn update(&mut self, stats: &SystemStats) {
        self.current = stats.cpu_usage;

        // Add to history
        if self.history.len() >= HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(self.current as u64);
    }
}

impl Default for CpuGaugeState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_cpu_gauge(frame: &mut Frame, state: &CpuGaugeState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" CPU ")
        .title_style(Style::default().fg(NEON_CYAN).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    // Split inner area for gauge and sparkline
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Render gauge
    let color = gauge_color(state.current);
    let gauge = Gauge::default()
        .ratio(state.current / 100.0)
        .gauge_style(Style::default().fg(color).bg(DARK_BG))
        .label(format!("{:.1}%", state.current));

    frame.render_widget(gauge, chunks[0]);

    // Render sparkline
    let data: Vec<u64> = state.history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(color));

    frame.render_widget(sparkline, chunks[1]);
}
