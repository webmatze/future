use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

/// Terminal events
#[derive(Debug)]
pub enum Event {
    /// Terminal tick (for animations)
    Tick,
    /// Key press
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
}

/// Handles terminal events
pub struct EventHandler {
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    /// Event sender (kept for potential future use)
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
}

impl EventHandler {
    /// Create a new event handler with specified tick rate in milliseconds
    pub fn new(tick_rate_ms: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let (sender, receiver) = mpsc::channel();
        let event_sender = sender.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // Calculate timeout until next tick
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::ZERO);

                // Poll for events
                if event::poll(timeout).expect("Failed to poll events") {
                    match event::read().expect("Failed to read event") {
                        CrosstermEvent::Key(key) => {
                            if event_sender.send(Event::Key(key)).is_err() {
                                return;
                            }
                        }
                        CrosstermEvent::Mouse(mouse) => {
                            if event_sender.send(Event::Mouse(mouse)).is_err() {
                                return;
                            }
                        }
                        CrosstermEvent::Resize(width, height) => {
                            if event_sender.send(Event::Resize(width, height)).is_err() {
                                return;
                            }
                        }
                        _ => {}
                    }
                }

                // Send tick event if enough time has passed
                if last_tick.elapsed() >= tick_rate {
                    if event_sender.send(Event::Tick).is_err() {
                        return;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver, sender }
    }

    /// Get the next event (blocking)
    pub fn next(&self) -> io::Result<Event> {
        self.receiver
            .recv()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
