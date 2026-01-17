use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Gauge},
};
use rand::Rng;

use crate::ui::theme::*;

const NUM_BARS: usize = 4;

struct ProgressBar {
    label: &'static str,
    progress: f64,
    speed: f64,
    color: Color,
    complete_flash: u8,
}

impl ProgressBar {
    fn new(label: &'static str, color: Color) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            label,
            progress: rng.gen_range(0.0..0.5),
            speed: rng.gen_range(0.002..0.015),
            color,
            complete_flash: 0,
        }
    }

    fn tick(&mut self) {
        if self.complete_flash > 0 {
            self.complete_flash -= 1;
            if self.complete_flash == 0 {
                self.reset();
            }
            return;
        }

        self.progress += self.speed;

        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.complete_flash = 30; // Flash for 30 ticks
        }
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.progress = 0.0;
        self.speed = rng.gen_range(0.002..0.015);
        self.label = random_operation();
    }
}

pub struct ProgressBarsState {
    bars: Vec<ProgressBar>,
}

impl ProgressBarsState {
    pub fn new() -> Self {
        let bars = vec![
            ProgressBar::new("DECRYPTING", NEON_CYAN),
            ProgressBar::new("UPLOADING", NEON_MAGENTA),
            ProgressBar::new("COMPILING", NEON_GREEN),
            ProgressBar::new("ANALYZING", NEON_ORANGE),
        ];

        Self { bars }
    }

    pub fn tick(&mut self) {
        for bar in &mut self.bars {
            bar.tick();
        }
    }
}

impl Default for ProgressBarsState {
    fn default() -> Self {
        Self::new()
    }
}

fn random_operation() -> &'static str {
    const OPERATIONS: &[&str] = &[
        "DECRYPTING",
        "ENCRYPTING",
        "UPLOADING",
        "DOWNLOADING",
        "COMPILING",
        "ANALYZING",
        "SCANNING",
        "INJECTING",
        "EXTRACTING",
        "PROCESSING",
        "DEPLOYING",
        "SYNCING",
        "HASHING",
        "CRACKING",
        "TUNNELING",
    ];

    let mut rng = rand::thread_rng();
    OPERATIONS[rng.gen_range(0..OPERATIONS.len())]
}

pub fn render_progress_bars(frame: &mut Frame, state: &ProgressBarsState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" OPERATIONS ")
        .title_style(Style::default().fg(NEON_PURPLE).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < NUM_BARS as u16 * 2 {
        return;
    }

    let bar_height = inner.height / NUM_BARS as u16;

    for (i, bar) in state.bars.iter().enumerate() {
        let bar_area = Rect {
            x: inner.x,
            y: inner.y + (i as u16 * bar_height),
            width: inner.width,
            height: bar_height.min(2),
        };

        let color = if bar.complete_flash > 0 {
            if bar.complete_flash % 6 < 3 {
                STATUS_SUCCESS
            } else {
                bar.color
            }
        } else {
            bar.color
        };

        let label = if bar.complete_flash > 0 {
            format!("{} ✓ COMPLETE", bar.label)
        } else {
            format!("{} {:5.1}%", bar.label, bar.progress * 100.0)
        };

        let gauge = Gauge::default()
            .ratio(bar.progress)
            .gauge_style(Style::default().fg(color).bg(DARK_BG))
            .label(label);

        frame.render_widget(gauge, bar_area);
    }
}
