use std::fs;
use std::process::Command;
use std::env;

pub fn get_os() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(content) => {
            let mut pretty_name = None;
            let mut name = None;
            let mut version = None;

            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    pretty_name = line.split('=').nth(1);
                }
                if line.starts_with("NAME=") {
                    name = line.split('=').nth(1);
                }
                if line.starts_with("VERSION=") {
                    version = line.split('=').nth(1);
                }
            }

            match (pretty_name, name) {
                (Some(p), _) => p.trim().trim_matches('"').to_string(),
                (None, Some(n)) => format!("{} {}", n.trim().trim_matches('"').to_string(), version.unwrap_or("Unknown version").trim().trim_matches('"').to_string()),
                (None, None) => "Linux (Unknown distro)".to_string(),
            }
        }
        Err(e) => format!("Unknown OS (Error: {})", e),
    }
}

pub fn get_hostname() -> String {
    match fs::read_to_string("/etc/hostname") {
        Ok(hostname) => hostname.trim().to_string(),
        Err(e) => format!("Unknown (Error: {})", e),
    }
}

pub fn get_cpu() -> String {
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(content) => {
            for line in content.lines() {
                if line.starts_with("model name") {
                    return line.split(':').nth(1).unwrap_or("Unknown").trim().to_string();
                }
            }
            "Unknown CPU".to_string()
        }
        Err(e) => format!("Unknown (Error reading /proc/cpuinfo: {})", e),
    }
}

pub fn get_memory() -> String {
    match fs::read_to_string("/proc/meminfo") {
        Ok(content) => {
            let mut mem_total = 0;
            let mut mem_available = 0;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    mem_total = line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                }
                if line.starts_with("MemAvailable:") {
                    mem_available = line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                }
            }

            format!("{} / {}", format_memory(mem_available * 1024), format_memory(mem_total * 1024)).to_string()
        }
        Err(e) => format!("Unknown (Error reading /proc/meminfo: {})", e),
    }
}

fn format_memory(input: u64) -> String {
    let units = ["", "K", "M", "G", "T", "P", "E"];
    let factor = (input.ilog10() / 3) as usize;
    let factor = factor.min(units.len() - 1);

    let value = input as f64 / 1024_f64.powi(factor as i32);

    if value.fract() < 0.01 {
        format!("{} {}B", value.round(), units[factor])
    } else {
        format!("{:.2} {}B", value, units[factor])
    }
}

pub fn get_uptime() -> String {
    match fs::read_to_string("/proc/uptime") {
        Ok(content) => {
            let uptime_secs = content
                .split_whitespace()
                .next()
                .unwrap_or("0.0")
                .parse::<f64>()
                .unwrap_or(0.0);

            let days = (uptime_secs / 86400.0).floor() as u64;
            let hours = ((uptime_secs % 86400.0) / 3600.0).floor() as u64;
            let mins = ((uptime_secs % 3600.0) / 60.0).floor() as u64;
            let secs = (uptime_secs % 60.0).floor() as u64;

            let mut parts = Vec::new();
            if days > 0 {
                parts.push(format!("{} day{}", days, if days > 1 { "s" } else { "" }));
            }
            if hours > 0 {
                parts.push(format!("{} hour{}", hours, if hours > 1 { "s" } else { "" }));
            }
            if mins > 0 {
                parts.push(format!("{} min{}", mins, if mins > 1 { "s" } else { "" }));
            }
            if secs > 0 {
                parts.push(format!("{} sec{}", secs, if secs > 1 { "s" } else { "" }));
            }

            parts.join(", ")
        }
        Err(e) => format!("Unknown (Error reading /proc/uptime: {})", e),
    }
}

pub fn get_kernel() -> String {
    match fs::read_to_string("/proc/version") {
        Ok(content) => content.split_whitespace()
            .nth(2)
            .unwrap_or("Unknown")
            .trim()
            .trim_matches('"')
            .to_string(),
        Err(e) => format!("Unknown (Error: {})", e),
    }
}

pub fn get_packages() -> String {
    match Command::new("dpkg").arg("-l").output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let count = stdout.lines()
                    .filter(|line| line.starts_with("ii"))
                    .count();
                format!("{} (dpkg)", count)
            } else {
                "Unknown (dpkg failed)".to_string()
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub fn get_shell() -> String {
    match env::var("SHELL") {
        Ok(shell_path) => {
            shell_path.split('/').last().unwrap_or("Unknown").to_string()
        }
        Err(e) => format!("Error: {}", e),
    }
}
