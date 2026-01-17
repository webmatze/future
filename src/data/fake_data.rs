use rand::Rng;

/// Generate a random IP address
pub fn random_ip() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(1..255)
    )
}

/// Generate a random port
pub fn random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024..65535)
}

/// Generate a random hex string
pub fn random_hex(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| format!("{:02X}", rng.gen::<u8>()))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a random percentage
pub fn random_percentage() -> f64 {
    rand::thread_rng().gen_range(0.0..100.0)
}

/// Generate fake process names
pub fn random_process() -> &'static str {
    const PROCESSES: &[&str] = &[
        "sshd", "nginx", "postgres", "redis", "docker",
        "node", "python3", "java", "rustc", "gcc",
        "systemd", "cron", "kernel", "init", "bash",
        "vim", "tmux", "htop", "curl", "wget",
    ];
    let mut rng = rand::thread_rng();
    PROCESSES[rng.gen_range(0..PROCESSES.len())]
}

/// Generate fake file paths
pub fn random_path() -> String {
    const DIRS: &[&str] = &[
        "/usr/bin", "/var/log", "/etc", "/home/user",
        "/opt/app", "/tmp", "/root", "/srv/data",
    ];
    const FILES: &[&str] = &[
        "config.json", "data.db", "server.log", "auth.key",
        "backup.tar.gz", "index.html", "main.py", "app.js",
    ];

    let mut rng = rand::thread_rng();
    format!(
        "{}/{}",
        DIRS[rng.gen_range(0..DIRS.len())],
        FILES[rng.gen_range(0..FILES.len())]
    )
}

/// Generate fake usernames
pub fn random_username() -> &'static str {
    const USERS: &[&str] = &[
        "root", "admin", "user", "guest", "daemon",
        "nobody", "www-data", "postgres", "redis", "neo",
    ];
    let mut rng = rand::thread_rng();
    USERS[rng.gen_range(0..USERS.len())]
}

/// Generate fake city names for the world map
pub fn random_city() -> &'static str {
    const CITIES: &[&str] = &[
        "New York", "London", "Tokyo", "Berlin", "Sydney",
        "Moscow", "Shanghai", "Paris", "Dubai", "Singapore",
    ];
    let mut rng = rand::thread_rng();
    CITIES[rng.gen_range(0..CITIES.len())]
}

/// Generate dramatic hacker-style messages
pub fn dramatic_message() -> &'static str {
    const MESSAGES: &[&str] = &[
        "ACCESS GRANTED",
        "FIREWALL BYPASSED",
        "ENCRYPTION CRACKED",
        "TARGET ACQUIRED",
        "INTRUSION DETECTED",
        "TRACE INITIATED",
        "PAYLOAD DEPLOYED",
        "BACKDOOR INSTALLED",
        "DATA EXFILTRATION COMPLETE",
        "SYSTEM COMPROMISED",
        "ROOT ACCESS OBTAINED",
        "AUTHENTICATION BYPASSED",
        "PROTOCOL OVERRIDE",
        "SECURITY BREACH",
        "MAINFRAME CONNECTED",
    ];
    let mut rng = rand::thread_rng();
    MESSAGES[rng.gen_range(0..MESSAGES.len())]
}

/// Katakana and other characters for Matrix rain
pub fn matrix_chars() -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();

    // Half-width katakana (U+FF66 to U+FF9D)
    for c in '\u{FF66}'..='\u{FF9D}' {
        chars.push(c);
    }

    // Numbers
    for c in '0'..='9' {
        chars.push(c);
    }

    // Some ASCII
    for c in ['@', '#', '$', '%', '&', '*', '+', '=', '<', '>', '/', '\\'] {
        chars.push(c);
    }

    chars
}

/// Generate code snippets for the source code widget
pub fn code_snippets() -> Vec<&'static str> {
    vec![
        r#"fn decrypt_payload(data: &[u8]) -> Result<Vec<u8>> {
    let key = derive_key(MASTER_SECRET);
    let cipher = Aes256Gcm::new(&key);
    cipher.decrypt(data)
}

async fn establish_connection(target: &str) {
    let socket = TcpStream::connect(target).await?;
    let (rx, tx) = socket.split();
    spawn_handler(rx, tx).await;
}"#,
        r#"class NetworkScanner:
    def __init__(self, subnet):
        self.subnet = subnet
        self.targets = []

    async def scan_range(self):
        for ip in self.subnet.hosts():
            if await self.probe(ip):
                self.targets.append(ip)
        return self.targets"#,
        r#"SELECT u.username, a.last_login, p.permissions
FROM users u
JOIN auth_sessions a ON u.id = a.user_id
JOIN permissions p ON u.role_id = p.role_id
WHERE a.active = true
  AND p.level >= 5
ORDER BY a.last_login DESC;"#,
        r#"#!/bin/bash
for host in $(cat targets.txt); do
    nmap -sV -p 22,80,443 $host >> scan.log
    if grep -q "open" scan.log; then
        ./exploit.sh $host &
    fi
done"#,
        r#"void inject_shellcode(HANDLE proc, LPVOID base) {
    SIZE_T written;
    WriteProcessMemory(proc, base, shellcode,
        sizeof(shellcode), &written);
    CreateRemoteThread(proc, NULL, 0,
        (LPTHREAD_START_ROUTINE)base, NULL, 0, NULL);
}"#,
    ]
}
