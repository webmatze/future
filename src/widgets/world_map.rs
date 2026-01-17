use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle, Line, Map, MapResolution},
        Block, Borders, BorderType,
    },
};
use rand::Rng;

use crate::ui::theme::*;

/// Major city nodes on the world map
#[derive(Clone)]
pub struct MapNode {
    pub name: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub active: bool,
    pub blink_phase: f32,
}

/// Connection between two nodes
#[derive(Clone)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub progress: f32,
    pub active: bool,
}

pub struct WorldMapState {
    pub nodes: Vec<MapNode>,
    pub connections: Vec<Connection>,
    tick_counter: u64,
}

impl WorldMapState {
    pub fn new() -> Self {
        let nodes = vec![
            MapNode { name: "NYC", lat: 40.7128, lon: -74.0060, active: true, blink_phase: 0.0 },
            MapNode { name: "LON", lat: 51.5074, lon: -0.1278, active: true, blink_phase: 0.3 },
            MapNode { name: "TYO", lat: 35.6762, lon: 139.6503, active: true, blink_phase: 0.6 },
            MapNode { name: "SFO", lat: 37.7749, lon: -122.4194, active: false, blink_phase: 0.1 },
            MapNode { name: "SYD", lat: -33.8688, lon: 151.2093, active: true, blink_phase: 0.4 },
            MapNode { name: "MOW", lat: 55.7558, lon: 37.6173, active: false, blink_phase: 0.7 },
            MapNode { name: "SHA", lat: 31.2304, lon: 121.4737, active: true, blink_phase: 0.2 },
            MapNode { name: "BER", lat: 52.5200, lon: 13.4050, active: true, blink_phase: 0.5 },
            MapNode { name: "DXB", lat: 25.2048, lon: 55.2708, active: false, blink_phase: 0.8 },
            MapNode { name: "SIN", lat: 1.3521, lon: 103.8198, active: true, blink_phase: 0.9 },
        ];

        // Initial connections
        let connections = vec![
            Connection { from: 0, to: 1, progress: 1.0, active: true },  // NYC -> LON
            Connection { from: 1, to: 7, progress: 1.0, active: true },  // LON -> BER
            Connection { from: 7, to: 5, progress: 0.0, active: false }, // BER -> MOW
            Connection { from: 2, to: 6, progress: 1.0, active: true },  // TYO -> SHA
            Connection { from: 6, to: 9, progress: 0.5, active: true },  // SHA -> SIN
            Connection { from: 9, to: 8, progress: 0.0, active: false }, // SIN -> DXB
            Connection { from: 3, to: 0, progress: 1.0, active: true },  // SFO -> NYC
            Connection { from: 4, to: 9, progress: 0.7, active: true },  // SYD -> SIN
        ];

        Self {
            nodes,
            connections,
            tick_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
        let mut rng = rand::thread_rng();

        // Update node blink phases
        for node in &mut self.nodes {
            node.blink_phase = (node.blink_phase + 0.05) % 1.0;

            // Randomly toggle active state
            if rng.gen_bool(0.002) {
                node.active = !node.active;
            }
        }

        // Update connections
        for conn in &mut self.connections {
            if conn.active {
                conn.progress = (conn.progress + 0.02).min(1.0);

                // Reset completed connections occasionally
                if conn.progress >= 1.0 && rng.gen_bool(0.01) {
                    conn.progress = 0.0;
                    conn.active = rng.gen_bool(0.7);
                }
            } else {
                // Randomly activate inactive connections
                if rng.gen_bool(0.005) {
                    conn.active = true;
                    conn.progress = 0.0;
                }
            }
        }

        // Occasionally add new random connection
        if rng.gen_bool(0.002) && self.connections.len() < 15 {
            let from = rng.gen_range(0..self.nodes.len());
            let to = rng.gen_range(0..self.nodes.len());
            if from != to {
                self.connections.push(Connection {
                    from,
                    to,
                    progress: 0.0,
                    active: true,
                });
            }
        }

        // Remove old completed connections
        if self.connections.len() > 12 {
            self.connections.retain(|c| c.active || c.progress < 1.0);
        }
    }
}

impl Default for WorldMapState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_world_map(frame: &mut Frame, state: &WorldMapState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" GLOBAL NETWORK ")
        .title_style(Style::default().fg(MAP_NODE_ACTIVE).bold());

    let canvas = Canvas::default()
        .block(block)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            // Draw the world map
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: MAP_OUTLINE,
            });

            // Draw connections
            for conn in &state.connections {
                if !conn.active && conn.progress == 0.0 {
                    continue;
                }

                let from = &state.nodes[conn.from];
                let to = &state.nodes[conn.to];

                // Calculate current endpoint based on progress
                let current_lon = from.lon + (to.lon - from.lon) * conn.progress as f64;
                let current_lat = from.lat + (to.lat - from.lat) * conn.progress as f64;

                // Draw the connection line
                ctx.draw(&Line {
                    x1: from.lon,
                    y1: from.lat,
                    x2: current_lon,
                    y2: current_lat,
                    color: if conn.active { MAP_CONNECTION } else { BORDER_DIM },
                });
            }

            // Draw nodes
            for node in &state.nodes {
                // Determine color based on active state and blink phase
                let blink = (node.blink_phase * std::f32::consts::PI * 2.0).sin();
                let color = if node.active {
                    if blink > 0.0 {
                        MAP_NODE_ACTIVE
                    } else {
                        Color::Rgb(0, 180, 180)
                    }
                } else {
                    MAP_NODE_IDLE
                };

                // Draw node circle
                ctx.draw(&Circle {
                    x: node.lon,
                    y: node.lat,
                    radius: if node.active { 3.0 } else { 2.0 },
                    color,
                });

                // Draw node label
                ctx.print(
                    node.lon + 2.0,
                    node.lat + 2.0,
                    Span::styled(node.name, Style::default().fg(color)),
                );
            }
        });

    frame.render_widget(canvas, area);
}
