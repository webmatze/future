use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Sparkline},
};
use std::collections::VecDeque;

use crate::data::system_stats::{format_bytes_per_sec, SystemStats};
use crate::ui::theme::*;

const HISTORY_SIZE: usize = 30;

pub struct NetworkMonitorState {
    pub rx_rate: u64,
    pub tx_rate: u64,
    pub rx_history: VecDeque<u64>,
    pub tx_history: VecDeque<u64>,
}

impl NetworkMonitorState {
    pub fn new() -> Self {
        let mut rx_history = VecDeque::with_capacity(HISTORY_SIZE);
        let mut tx_history = VecDeque::with_capacity(HISTORY_SIZE);

        for _ in 0..HISTORY_SIZE {
            rx_history.push_back(0);
            tx_history.push_back(0);
        }

        Self {
            rx_rate: 0,
            tx_rate: 0,
            rx_history,
            tx_history,
        }
    }

    pub fn update(&mut self, stats: &SystemStats) {
        self.rx_rate = stats.network_rx;
        self.tx_rate = stats.network_tx;

        // Update histories
        if self.rx_history.len() >= HISTORY_SIZE {
            self.rx_history.pop_front();
        }
        self.rx_history.push_back(self.rx_rate);

        if self.tx_history.len() >= HISTORY_SIZE {
            self.tx_history.pop_front();
        }
        self.tx_history.push_back(self.tx_rate);
    }
}

impl Default for NetworkMonitorState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_network_monitor(frame: &mut Frame, state: &NetworkMonitorState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" NETWORK ")
        .title_style(Style::default().fg(NEON_ORANGE).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 4 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // TX label
            Constraint::Length(1), // TX sparkline
            Constraint::Length(1), // RX label
            Constraint::Min(1),    // RX sparkline
        ])
        .split(inner);

    // TX (upload)
    let tx_label = Line::from(vec![
        Span::styled("▲ TX: ", Style::default().fg(NEON_ORANGE)),
        Span::styled(
            format_bytes_per_sec(state.tx_rate),
            Style::default().fg(TEXT_PRIMARY),
        ),
    ]);
    frame.render_widget(ratatui::widgets::Paragraph::new(tx_label), chunks[0]);

    let tx_data: Vec<u64> = state.tx_history.iter().copied().collect();
    let tx_sparkline = Sparkline::default()
        .data(&tx_data)
        .style(Style::default().fg(NEON_ORANGE));
    frame.render_widget(tx_sparkline, chunks[1]);

    // RX (download)
    let rx_label = Line::from(vec![
        Span::styled("▼ RX: ", Style::default().fg(NEON_CYAN)),
        Span::styled(
            format_bytes_per_sec(state.rx_rate),
            Style::default().fg(TEXT_PRIMARY),
        ),
    ]);
    frame.render_widget(ratatui::widgets::Paragraph::new(rx_label), chunks[2]);

    let rx_data: Vec<u64> = state.rx_history.iter().copied().collect();
    let rx_sparkline = Sparkline::default()
        .data(&rx_data)
        .style(Style::default().fg(NEON_CYAN));
    frame.render_widget(rx_sparkline, chunks[3]);
}
