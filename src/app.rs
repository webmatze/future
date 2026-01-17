use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::widgets::{
    clock::ClockState,
    countdown_timer::CountdownState,
    cpu_gauge::CpuGaugeState,
    fake_logs::FakeLogsState,
    hex_dump::HexDumpState,
    matrix_rain::MatrixRainState,
    memory_gauge::MemoryGaugeState,
    network_monitor::NetworkMonitorState,
    progress_bars::ProgressBarsState,
    source_code::SourceCodeState,
    world_map::WorldMapState,
};
use crate::data::system_stats::SystemStats;

/// Application state
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Is animation paused?
    pub paused: bool,
    /// Current tick count
    pub tick_count: u64,
    /// Terminal size
    pub terminal_size: (u16, u16),
    /// Animation speed multiplier (1.0 = normal)
    pub animation_speed: f32,
    /// Show help overlay
    pub show_help: bool,

    // System stats
    pub system_stats: SystemStats,

    // Widget states
    pub matrix_state: MatrixRainState,
    pub logs_state: FakeLogsState,
    pub source_state: SourceCodeState,
    pub cpu_state: CpuGaugeState,
    pub memory_state: MemoryGaugeState,
    pub network_state: NetworkMonitorState,
    pub map_state: WorldMapState,
    pub countdown_state: CountdownState,
    pub clock_state: ClockState,
    pub hex_state: HexDumpState,
    pub progress_state: ProgressBarsState,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            paused: false,
            tick_count: 0,
            terminal_size: (80, 24),
            animation_speed: 1.0,
            show_help: false,
            system_stats: SystemStats::new(),
            matrix_state: MatrixRainState::new(),
            logs_state: FakeLogsState::new(),
            source_state: SourceCodeState::new(),
            cpu_state: CpuGaugeState::new(),
            memory_state: MemoryGaugeState::new(),
            network_state: NetworkMonitorState::new(),
            map_state: WorldMapState::new(),
            countdown_state: CountdownState::new(300), // 5 minute countdown
            clock_state: ClockState::new(),
            hex_state: HexDumpState::new(),
            progress_state: ProgressBarsState::new(),
        }
    }

    /// Process a tick - update all animations
    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        self.tick_count += 1;

        // Update system stats every ~60 ticks (1 second at 60 FPS)
        if self.tick_count % 60 == 0 {
            self.system_stats.refresh();
            self.cpu_state.update(&self.system_stats);
            self.memory_state.update(&self.system_stats);
            self.network_state.update(&self.system_stats);
        }

        // Update animated widgets
        self.matrix_state.tick();
        self.logs_state.tick();
        self.source_state.tick();
        self.map_state.tick();
        self.countdown_state.tick();
        self.clock_state.tick();
        self.hex_state.tick();
        self.progress_state.tick();
    }

    /// Handle keyboard input
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        // Close help overlay first if open
        if self.show_help {
            self.show_help = false;
            return;
        }

        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            // Quit with Ctrl+C
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.running = false;
            }
            // Pause/Resume
            KeyCode::Char(' ') => {
                self.paused = !self.paused;
            }
            // Speed controls
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.animation_speed = (self.animation_speed + 0.25).min(3.0);
            }
            KeyCode::Char('-') => {
                self.animation_speed = (self.animation_speed - 0.25).max(0.25);
            }
            // Reset countdown
            KeyCode::Char('r') => {
                self.countdown_state.reset();
            }
            // Help
            KeyCode::Char('?') | KeyCode::Char('h') => {
                self.show_help = true;
            }
            _ => {}
        }
    }

    /// Handle terminal resize
    pub fn handle_resize(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
        self.matrix_state.resize(width, height);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
