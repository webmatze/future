use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
};
use chrono::Local;
use rand::Rng;
use std::collections::VecDeque;

use crate::data::fake_data::{dramatic_message, random_ip, random_path, random_port, random_process, random_username};
use crate::ui::theme::*;

const MAX_LOGS: usize = 100;

#[derive(Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
    Debug,
    Alert,
}

impl LogLevel {
    fn color(&self) -> Color {
        match self {
            LogLevel::Info => LOG_INFO,
            LogLevel::Warn => LOG_WARN,
            LogLevel::Error => LOG_ERROR,
            LogLevel::Success => LOG_SUCCESS,
            LogLevel::Debug => LOG_DEBUG,
            LogLevel::Alert => NEON_MAGENTA,
        }
    }

    fn label(&self) -> &str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERR!",
            LogLevel::Success => "OK",
            LogLevel::Debug => "DBG",
            LogLevel::Alert => "ALERT",
        }
    }
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

pub struct FakeLogsState {
    pub logs: VecDeque<LogEntry>,
    tick_counter: u64,
    next_log_at: u64,
}

impl FakeLogsState {
    pub fn new() -> Self {
        let mut state = Self {
            logs: VecDeque::with_capacity(MAX_LOGS),
            tick_counter: 0,
            next_log_at: 0,
        };

        // Add some initial logs
        for _ in 0..5 {
            state.add_random_log();
        }

        state
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;

        if self.tick_counter >= self.next_log_at {
            self.add_random_log();
            // Schedule next log between 10 and 60 ticks (roughly 0.16s to 1s at 60fps)
            self.next_log_at = self.tick_counter + rand::thread_rng().gen_range(10..60);
        }
    }

    fn add_random_log(&mut self) {
        let mut rng = rand::thread_rng();

        // Occasionally add a dramatic alert
        if rng.gen_bool(0.05) {
            self.add_log(LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Alert,
                message: dramatic_message().to_string(),
            });
            return;
        }

        let entry = match rng.gen_range(0..10) {
            0 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Info,
                message: format!("Connection established from {}", random_ip()),
            },
            1 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Info,
                message: format!("Process {} spawned on port {}", random_process(), random_port()),
            },
            2 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Warn,
                message: format!("Authentication attempt for user '{}'", random_username()),
            },
            3 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Error,
                message: format!("Failed to access {}", random_path()),
            },
            4 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Success,
                message: format!("Encrypted tunnel to {} active", random_ip()),
            },
            5 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Debug,
                message: format!("Scanning port range {}-{}", random_port(), random_port()),
            },
            6 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Info,
                message: format!("Data packet received: {} bytes", rng.gen_range(64..65536)),
            },
            7 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Warn,
                message: format!("Firewall rule triggered from {}", random_ip()),
            },
            8 => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Info,
                message: format!("Decrypting sector 0x{:08X}...", rng.gen::<u32>()),
            },
            _ => LogEntry {
                timestamp: Local::now().format("%H:%M:%S%.3f").to_string(),
                level: LogLevel::Debug,
                message: format!("Memory allocation: {} KB", rng.gen_range(1..1024)),
            },
        };

        self.add_log(entry);
    }

    fn add_log(&mut self, entry: LogEntry) {
        if self.logs.len() >= MAX_LOGS {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
    }
}

impl Default for FakeLogsState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_fake_logs(frame: &mut Frame, state: &FakeLogsState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" LOGS ")
        .title_style(Style::default().fg(NEON_GREEN).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Convert logs to lines, showing most recent at bottom
    let visible_logs = state.logs.iter().rev().take(inner.height as usize).rev();

    let lines: Vec<Line> = visible_logs
        .map(|entry| {
            Line::from(vec![
                Span::styled(
                    format!("{} ", entry.timestamp),
                    Style::default().fg(TEXT_DIM),
                ),
                Span::styled(
                    format!("[{}] ", entry.level.label()),
                    Style::default().fg(entry.level.color()).bold(),
                ),
                Span::styled(&entry.message, Style::default().fg(TEXT_PRIMARY)),
            ])
        })
        .collect();

    let logs_widget = Paragraph::new(lines).wrap(Wrap { trim: true });

    frame.render_widget(logs_widget, inner);
}
