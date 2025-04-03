use std::fs;
use std::process::Command as cmd;
use std::env;

pub fn get_os() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(content) => {
            let mut pretty_name: Option<String> = None;
            let mut name: Option<String> = None;
            let mut version: Option<String> = None;

            for line in content.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    pretty_name = Some(value.trim().trim_matches('"').to_string());
                }
                if let Some(value) = line.strip_prefix("NAME=") {
                    name = Some(value.trim().trim_matches('"').to_string());
                }
                if let Some(value) = line.strip_prefix("VERSION=") {
                    version = Some(value.trim().trim_matches('"').to_string());
                }
            }

            match (pretty_name, name) {
                (Some(p), _) => p,
                (None, Some(n)) => format!("{} {}", n, version.unwrap_or("Unknown version".to_string())),
                (None, None) => "Unknown distro".to_string(),
            }
        }
        Err(err) => format!("Unknown OS: {}", err),
    }
}

pub fn get_hostname() -> String {
    match fs::read_to_string("/etc/hostname") {
        Ok(hostname) => hostname.trim().to_string(),
        Err(err) => format!("Hostname error: {}", err),
    }
}

pub fn get_uptime() -> String {
    match fs::read_to_string("/proc/uptime") {
        Ok(content) => {
            let uptime_secs = content
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);

            let days = (uptime_secs / 86400.0).floor() as u64;
            let hours = ((uptime_secs % 86400.0) / 3600.0).floor() as u64;
            let mins = ((uptime_secs % 3600.0) / 60.0).floor() as u64;
            let secs = (uptime_secs % 60.0).floor() as u64;

            let mut parts = Vec::new();
            if days > 0 {
                parts.push(pluralize(days, "day"));
            }
            if hours > 0 {
                parts.push(pluralize(hours, "hour"));
            }
            if mins > 0 {
                parts.push(pluralize(mins, "min"));
            }
            if secs > 0 || parts.is_empty() {
                parts.push(pluralize(secs, "sec"));
            }

            parts.join(", ")
        }
        Err(err) => format!("Error reading '/proc/uptime': {}", err),
    }
}

fn pluralize(value: u64, unit: &str) -> String {
    format!("{} {}{}", value, unit, if value > 1 { "s" } else { "" })
}

pub fn get_kernel() -> String {
    match fs::read_to_string("/proc/version") {
        Ok(content) => content.split_whitespace()
            .nth(2)
            .map(|kernel| kernel.trim().trim_matches('"').to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        Err(err) => format!("Error reading '/proc/version': {}", err),
    }
}

pub fn get_packages() -> String {
    match cmd::new("dpkg").arg("-l").output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let count = stdout.lines()
                    .filter(|line| line.starts_with("ii"))
                    .count();
                format!("{} (dpkg)", count)
            } else {
                format!("dpkg failed with status: {}", output.status)
            }
        }
        Err(err) => format!("Error executing dpkg: {}", err),
    }
}

pub fn get_shell() -> String {
    match env::var("SHELL") {
        Ok(shell_path) => {
            shell_path.split('/').last().unwrap_or("N/A").to_string()
        }
        Err(_) => "SHELL environment variable not set".to_string(),
    }
}

pub fn get_user() -> String {
    match env::var("USER") {
        Ok(user) => user,
        Err(_) => match cmd::new("whoami").output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(err) => format!("Error executing whoami: {}", err),
        },
    }
}

pub fn get_network() -> String {
    cmd::new("ip")
        .args(["-4", "addr", "show"])
        .output()
        .map(|output| {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str
                    .lines()
                    .filter(|line| line.contains("inet") && !line.contains("127.0.0.1"))
                    .next()
                    .map(|line| {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let ip = parts[1].split('/').next().unwrap_or("Unknown IP");
                            let interface = parts.get(parts.len() - 1).unwrap_or(&"Unknown interface");
                            format!("{}: {}", interface, ip)
                        } else {
                            "Unknown network".to_string()
                        }
                    })
                    .unwrap_or("No active network".to_string())
            } else {
                "Network command failed".to_string()
            }
        })
        .unwrap_or_else(|_| "Error fetching network info".to_string())
}
