use std::fs;
use crate::theme;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn read_cpu_stats() -> Option<(u64, u64)> {
    let file = fs::File::open("/proc/stat").ok()?;
    let mut lines = BufReader::new(file).lines();
    let cpu_line = lines.next()?.ok()?;

    let parts: Vec<&str> = cpu_line.split_whitespace().collect();
    if parts.len() < 5 || parts[0] != "cpu" {
        return None;
    }

    let user: u64 = parts[1].parse().ok()?;
    let nice: u64 = parts[2].parse().ok()?;
    let system: u64 = parts[3].parse().ok()?;
    let idle: u64 = parts[4].parse().ok()?;
    let iowait: u64 = parts[5].parse().ok()?;

    let total = user + nice + system + idle + iowait;
    let idle = idle + iowait;
    
    Some((total, idle))
}

pub fn get_cpu_usage() -> String {
    let first = read_cpu_stats().unwrap_or((1, 0));
    sleep(Duration::from_secs(1));
    let second = read_cpu_stats().unwrap_or((1, 0));

    let total_diff = (second.0 as f64) - (first.0 as f64);
    let idle_diff = (second.1 as f64) - (first.1 as f64);
    let usage = if total_diff > 0.0 {
        ((total_diff - idle_diff) / total_diff) * 100.0
    } else {
        0.0
    };

    let bar_len = 20;
    let filled = (usage / 100.0 * bar_len as f64).round() as usize;
    let empty = bar_len - filled;

    format!(
        "[{}{}] ({})",
        theme::colorize(&"#".repeat(filled)),
        ".".repeat(empty),
        theme::colorize(&format!("{:.1}%", usage))
    )
}

pub fn get_cpu_info() -> String {
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(content) => {
            for line in content.lines() {
                if line.starts_with("model name") {
                    return line.split(':').nth(1).unwrap_or("N/A").trim().to_string();
                }
            }
            "N/A".to_string()
        }
        Err(err) => format!("Error reading '/proc/cpuinfo': {}", err),
    }
}
