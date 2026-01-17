use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
};
use rand::Rng;

use crate::ui::theme::*;

const BYTES_PER_LINE: usize = 16;
const MAX_LINES: usize = 100;

pub struct HexDumpState {
    lines: Vec<HexLine>,
    current_offset: u64,
    scroll_offset: usize,
    tick_counter: u64,
}

struct HexLine {
    offset: u64,
    bytes: Vec<u8>,
    highlight_idx: Option<usize>,
}

impl HexDumpState {
    pub fn new() -> Self {
        let mut state = Self {
            lines: Vec::with_capacity(MAX_LINES),
            current_offset: 0x7F3A0000,
            scroll_offset: 0,
            tick_counter: 0,
        };

        // Generate initial lines
        for _ in 0..20 {
            state.add_line();
        }

        state
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;

        // Add new line every 10 ticks
        if self.tick_counter % 10 == 0 {
            self.add_line();
            self.scroll_offset = self.lines.len().saturating_sub(10);
        }

        // Update highlight positions
        if self.tick_counter % 5 == 0 {
            let mut rng = rand::thread_rng();
            for line in &mut self.lines {
                if rng.gen_bool(0.1) {
                    line.highlight_idx = Some(rng.gen_range(0..line.bytes.len()));
                } else {
                    line.highlight_idx = None;
                }
            }
        }
    }

    fn add_line(&mut self) {
        let mut rng = rand::thread_rng();

        let bytes: Vec<u8> = (0..BYTES_PER_LINE)
            .map(|_| rng.gen())
            .collect();

        let line = HexLine {
            offset: self.current_offset,
            bytes,
            highlight_idx: None,
        };

        if self.lines.len() >= MAX_LINES {
            self.lines.remove(0);
        }

        self.lines.push(line);
        self.current_offset += BYTES_PER_LINE as u64;
    }
}

impl Default for HexDumpState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_hex_dump(frame: &mut Frame, state: &HexDumpState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" DATA STREAM ")
        .title_style(Style::default().fg(NEON_GREEN).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_lines = state.lines
        .iter()
        .skip(state.scroll_offset)
        .take(inner.height as usize);

    let lines: Vec<Line> = visible_lines
        .map(|hex_line| {
            let mut spans = vec![
                // Offset
                Span::styled(
                    format!("0x{:08X}  ", hex_line.offset),
                    Style::default().fg(TEXT_DIM),
                ),
            ];

            // Hex bytes
            for (i, byte) in hex_line.bytes.iter().enumerate() {
                let is_highlighted = hex_line.highlight_idx == Some(i);
                let color = if is_highlighted {
                    NEON_MAGENTA
                } else {
                    NEON_GREEN
                };

                spans.push(Span::styled(
                    format!("{:02X} ", byte),
                    Style::default().fg(color),
                ));

                // Add extra space in the middle
                if i == 7 {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            spans.push(Span::styled(" │ ", Style::default().fg(BORDER_DIM)));

            // ASCII representation
            for (i, &byte) in hex_line.bytes.iter().enumerate() {
                let ch = if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    '.'
                };

                let is_highlighted = hex_line.highlight_idx == Some(i);
                let color = if is_highlighted {
                    NEON_MAGENTA
                } else if byte.is_ascii_graphic() {
                    TEXT_PRIMARY
                } else {
                    TEXT_DIM
                };

                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(color),
                ));
            }

            Line::from(spans)
        })
        .collect();

    let hex_widget = Paragraph::new(lines);
    frame.render_widget(hex_widget, inner);
}
