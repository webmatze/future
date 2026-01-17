use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
};

use crate::app::App;
use crate::ui::theme::*;
use crate::widgets::{
    clock::render_clock,
    countdown_timer::render_countdown,
    cpu_gauge::render_cpu_gauge,
    fake_logs::render_fake_logs,
    hex_dump::render_hex_dump,
    matrix_rain::render_matrix_rain,
    memory_gauge::render_memory_gauge,
    network_monitor::render_network_monitor,
    progress_bars::render_progress_bars,
    source_code::render_source_code,
    world_map::render_world_map,
};

/// Create a neon-styled block with title
pub fn neon_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(format!(" {} ", title))
        .title_style(Style::default().fg(NEON_CYAN).bold())
}

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Check minimum size
    if area.width < 80 || area.height < 24 {
        render_size_warning(frame, area);
        return;
    }

    // Main vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header (clock, status, countdown)
            Constraint::Min(15),    // Main content area
            Constraint::Length(10), // Footer (hex dump, progress)
        ])
        .split(area);

    // Render header
    render_header(frame, app, main_chunks[0]);

    // Render main content
    render_main_content(frame, app, main_chunks[1]);

    // Render footer
    render_footer(frame, app, main_chunks[2]);

    // Render help overlay if active
    if app.show_help {
        render_help_overlay(frame, area);
    }

    // Render pause indicator
    if app.paused {
        render_pause_indicator(frame, area);
    }
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Clock
            Constraint::Percentage(50), // Status/Title
            Constraint::Percentage(25), // Countdown
        ])
        .split(area);

    render_clock(frame, &app.clock_state, header_chunks[0]);
    render_title(frame, header_chunks[1]);
    render_countdown(frame, &app.countdown_state, header_chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title_text = vec![
        Line::from(vec![
            Span::styled("F U T U R E   T E R M I N A L", Style::default().fg(NEON_MAGENTA).bold()),
        ]),
        Line::from(vec![
            Span::styled("SYSTEM ACTIVE • MONITORING", Style::default().fg(NEON_GREEN)),
        ]),
    ];

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(NEON_CYAN))
        );
    frame.render_widget(title, area);
}

fn render_main_content(frame: &mut Frame, app: &App, area: Rect) {
    // Split into three columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Left: Matrix + Source
            Constraint::Percentage(45), // Center: World Map
            Constraint::Percentage(30), // Right: System monitors + Logs
        ])
        .split(area);

    // Left column: Matrix Rain (top) + Source Code (bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(columns[0]);

    render_matrix_rain(frame, &app.matrix_state, left_chunks[0]);
    render_source_code(frame, &app.source_state, left_chunks[1]);

    // Center: World Map
    render_world_map(frame, &app.map_state, columns[1]);

    // Right column: System monitors (top) + Logs (bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),  // CPU
            Constraint::Length(4),  // Memory
            Constraint::Length(6),  // Network
            Constraint::Min(5),     // Logs
        ])
        .split(columns[2]);

    render_cpu_gauge(frame, &app.cpu_state, right_chunks[0]);
    render_memory_gauge(frame, &app.memory_state, right_chunks[1]);
    render_network_monitor(frame, &app.network_state, right_chunks[2]);
    render_fake_logs(frame, &app.logs_state, right_chunks[3]);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Hex dump
            Constraint::Percentage(40), // Progress bars
        ])
        .split(area);

    render_hex_dump(frame, &app.hex_state, footer_chunks[0]);
    render_progress_bars(frame, &app.progress_state, footer_chunks[1]);
}

fn render_size_warning(frame: &mut Frame, area: Rect) {
    let warning = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "⚠ TERMINAL TOO SMALL",
            Style::default().fg(NEON_ORANGE).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Minimum size: 80x24",
            Style::default().fg(TEXT_DIM),
        )),
        Line::from(Span::styled(
            format!("Current: {}x{}", area.width, area.height),
            Style::default().fg(TEXT_DIM),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(NEON_ORANGE)),
    );
    frame.render_widget(warning, area);
}

fn render_help_overlay(frame: &mut Frame, area: Rect) {
    let help_area = centered_rect(50, 60, area);

    frame.render_widget(Clear, help_area);

    let help_text = vec![
        Line::from(Span::styled(
            "═══ CONTROLS ═══",
            Style::default().fg(NEON_CYAN).bold(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  q / ESC  ", Style::default().fg(NEON_MAGENTA)),
            Span::styled("Quit", Style::default().fg(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  SPACE    ", Style::default().fg(NEON_MAGENTA)),
            Span::styled("Pause/Resume", Style::default().fg(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  + / -    ", Style::default().fg(NEON_MAGENTA)),
            Span::styled("Speed up/down", Style::default().fg(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  r        ", Style::default().fg(NEON_MAGENTA)),
            Span::styled("Reset countdown", Style::default().fg(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  ? / h    ", Style::default().fg(NEON_MAGENTA)),
            Span::styled("Toggle help", Style::default().fg(TEXT_PRIMARY)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press any key to close",
            Style::default().fg(TEXT_DIM),
        )),
    ];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(NEON_CYAN))
                .title(" HELP ")
                .title_style(Style::default().fg(NEON_MAGENTA).bold()),
        );

    frame.render_widget(help, help_area);
}

fn render_pause_indicator(frame: &mut Frame, area: Rect) {
    let pause_area = Rect {
        x: area.width.saturating_sub(12),
        y: 0,
        width: 12,
        height: 1,
    };

    let pause = Paragraph::new(Span::styled(
        " ⏸ PAUSED ",
        Style::default().fg(Color::Black).bg(NEON_ORANGE).bold(),
    ));

    frame.render_widget(pause, pause_area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
