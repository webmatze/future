use ratatui::style::Color;

// Primary accent colors - Cyberpunk Neon
pub const NEON_CYAN: Color = Color::Rgb(0, 255, 255);
pub const NEON_MAGENTA: Color = Color::Rgb(255, 0, 217);
pub const NEON_GREEN: Color = Color::Rgb(0, 255, 128);
pub const NEON_ORANGE: Color = Color::Rgb(255, 165, 0);
pub const NEON_PURPLE: Color = Color::Rgb(180, 0, 255);
pub const NEON_YELLOW: Color = Color::Rgb(255, 255, 0);
pub const NEON_RED: Color = Color::Rgb(255, 50, 50);

// Background colors
pub const DARK_BG: Color = Color::Rgb(10, 10, 20);
pub const PANEL_BG: Color = Color::Rgb(15, 15, 30);
pub const BORDER_DIM: Color = Color::Rgb(40, 40, 60);
pub const BORDER_ACTIVE: Color = Color::Rgb(0, 200, 255);

// Text colors
pub const TEXT_PRIMARY: Color = Color::Rgb(220, 220, 255);
pub const TEXT_DIM: Color = Color::Rgb(100, 100, 120);
pub const TEXT_HIGHLIGHT: Color = Color::Rgb(255, 255, 255);

// Status colors
pub const STATUS_SUCCESS: Color = Color::Rgb(0, 255, 100);
pub const STATUS_WARNING: Color = Color::Rgb(255, 200, 0);
pub const STATUS_ERROR: Color = Color::Rgb(255, 50, 50);
pub const STATUS_INFO: Color = Color::Rgb(0, 200, 255);

// Matrix Rain colors
pub const MATRIX_HEAD: Color = Color::Rgb(200, 255, 200);
pub const MATRIX_BODY: Color = Color::Rgb(0, 255, 0);
pub const MATRIX_TRAIL_1: Color = Color::Rgb(0, 200, 0);
pub const MATRIX_TRAIL_2: Color = Color::Rgb(0, 150, 0);
pub const MATRIX_TRAIL_3: Color = Color::Rgb(0, 100, 0);
pub const MATRIX_TRAIL_4: Color = Color::Rgb(0, 50, 0);

// World map colors
pub const MAP_OUTLINE: Color = Color::Rgb(30, 80, 30);
pub const MAP_NODE_IDLE: Color = Color::Rgb(0, 100, 100);
pub const MAP_NODE_ACTIVE: Color = Color::Rgb(0, 255, 255);
pub const MAP_CONNECTION: Color = Color::Rgb(0, 200, 200);

// Log level colors
pub const LOG_INFO: Color = Color::Rgb(0, 200, 255);
pub const LOG_WARN: Color = Color::Rgb(255, 200, 0);
pub const LOG_ERROR: Color = Color::Rgb(255, 80, 80);
pub const LOG_SUCCESS: Color = Color::Rgb(0, 255, 128);
pub const LOG_DEBUG: Color = Color::Rgb(150, 150, 150);

// Syntax highlighting colors
pub const SYNTAX_KEYWORD: Color = Color::Rgb(255, 0, 217);
pub const SYNTAX_STRING: Color = Color::Rgb(255, 165, 0);
pub const SYNTAX_COMMENT: Color = Color::Rgb(128, 128, 128);
pub const SYNTAX_FUNCTION: Color = Color::Rgb(0, 200, 255);
pub const SYNTAX_NUMBER: Color = Color::Rgb(255, 255, 0);
pub const SYNTAX_TYPE: Color = Color::Rgb(0, 255, 128);

// Gauge colors by usage level
pub fn gauge_color(percentage: f64) -> Color {
    if percentage < 50.0 {
        NEON_CYAN
    } else if percentage < 80.0 {
        NEON_YELLOW
    } else {
        NEON_RED
    }
}

// Progress bar gradient
pub fn progress_gradient(progress: f64) -> Color {
    let r = (255.0 * (1.0 - progress)) as u8;
    let g = (255.0 * progress) as u8;
    Color::Rgb(r, g, 128)
}
