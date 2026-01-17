use sysinfo::{System, Networks};

/// Real system statistics
pub struct SystemStats {
    system: System,
    networks: Networks,
    pub cpu_usage: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    last_rx: u64,
    last_tx: u64,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let networks = Networks::new_with_refreshed_list();

        let mut stats = Self {
            system,
            networks,
            cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            network_rx: 0,
            network_tx: 0,
            last_rx: 0,
            last_tx: 0,
        };

        stats.refresh();
        stats
    }

    pub fn refresh(&mut self) {
        // Refresh CPU
        self.system.refresh_cpu_usage();
        self.cpu_usage = self.system.global_cpu_usage() as f64;

        // Refresh memory
        self.system.refresh_memory();
        self.memory_used = self.system.used_memory();
        self.memory_total = self.system.total_memory();

        // Refresh network
        self.networks.refresh();

        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;

        for (_, data) in self.networks.iter() {
            total_rx = total_rx.saturating_add(data.total_received());
            total_tx = total_tx.saturating_add(data.total_transmitted());
        }

        // Calculate bytes per second (delta from last refresh)
        self.network_rx = total_rx.saturating_sub(self.last_rx);
        self.network_tx = total_tx.saturating_sub(self.last_tx);

        self.last_rx = total_rx;
        self.last_tx = total_tx;
    }

    pub fn memory_percentage(&self) -> f64 {
        if self.memory_total == 0 {
            return 0.0;
        }
        (self.memory_used as f64 / self.memory_total as f64) * 100.0
    }
}

impl Default for SystemStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format bytes per second
pub fn format_bytes_per_sec(bytes: u64) -> String {
    format!("{}/s", format_bytes(bytes))
}
