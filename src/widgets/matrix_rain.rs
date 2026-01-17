use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType},
};
use rand::Rng;

use crate::data::fake_data::matrix_chars;
use crate::ui::theme::*;

/// A single falling drop in the matrix rain
#[derive(Clone)]
struct Drop {
    x: u16,
    y: f32,
    speed: f32,
    length: usize,
    chars: Vec<char>,
}

impl Drop {
    fn new(x: u16, height: u16, char_set: &[char]) -> Self {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(5..15);
        let chars: Vec<char> = (0..length)
            .map(|_| char_set[rng.gen_range(0..char_set.len())])
            .collect();

        Self {
            x,
            y: -(rng.gen_range(0..height) as f32),
            speed: rng.gen_range(0.3..1.2),
            length,
            chars,
        }
    }

    fn tick(&mut self, height: u16, char_set: &[char]) {
        self.y += self.speed;

        // Reset when fully off screen
        if self.y > (height + self.length as u16) as f32 {
            let mut rng = rand::thread_rng();
            self.y = -(rng.gen_range(0..10) as f32);
            self.speed = rng.gen_range(0.3..1.2);
            self.length = rng.gen_range(5..15);
            self.chars = (0..self.length)
                .map(|_| char_set[rng.gen_range(0..char_set.len())])
                .collect();
        }

        // Randomly mutate some characters
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let idx = rng.gen_range(0..self.chars.len());
            self.chars[idx] = char_set[rng.gen_range(0..char_set.len())];
        }
    }
}

pub struct MatrixRainState {
    drops: Vec<Drop>,
    char_set: Vec<char>,
    width: u16,
    height: u16,
}

impl MatrixRainState {
    pub fn new() -> Self {
        let char_set = matrix_chars();

        Self {
            drops: Vec::new(),
            char_set,
            width: 0,
            height: 0,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;

        // Recreate drops for new dimensions
        let drop_count = (width / 2) as usize; // One drop every 2 columns
        self.drops = (0..drop_count)
            .map(|i| Drop::new((i * 2) as u16, height, &self.char_set))
            .collect();
    }

    pub fn tick(&mut self) {
        for drop in &mut self.drops {
            drop.tick(self.height, &self.char_set);
        }
    }
}

impl Default for MatrixRainState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_matrix_rain(frame: &mut Frame, state: &MatrixRainState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" MATRIX ")
        .title_style(Style::default().fg(MATRIX_BODY).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Create a buffer to render the matrix
    let buf = frame.buffer_mut();

    for drop in &state.drops {
        let x = inner.x + drop.x.min(inner.width.saturating_sub(1));

        for (i, &ch) in drop.chars.iter().enumerate() {
            let y = drop.y as i32 - i as i32;

            if y >= 0 && y < inner.height as i32 {
                let screen_y = inner.y + y as u16;

                if screen_y < inner.y + inner.height && x < inner.x + inner.width {
                    let color = if i == 0 {
                        MATRIX_HEAD
                    } else if i < 3 {
                        MATRIX_BODY
                    } else if i < 6 {
                        MATRIX_TRAIL_1
                    } else if i < 9 {
                        MATRIX_TRAIL_2
                    } else if i < 12 {
                        MATRIX_TRAIL_3
                    } else {
                        MATRIX_TRAIL_4
                    };

                    buf[(x, screen_y)]
                        .set_char(ch)
                        .set_fg(color);
                }
            }
        }
    }
}
