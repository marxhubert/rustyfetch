use std::fs;
use std::collections::HashMap;
use crate::theme;
use crate::utils;

const TARGET_KEYS: [&str; 3] = ["MemTotal:", "MemFree:", "MemAvailable:"];

fn mem_info() -> HashMap<String, u64> {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .map(|content| {
            content
                .lines()
                .filter_map(|line| {
                    let mut parts = line.split_whitespace();
                    let key = parts.next()?;
                    let value = parts.next()?.parse::<u64>().ok()?;

                    if TARGET_KEYS.contains(&key) {
                        Some((key.trim_end_matches(':').to_string(), value * 1024))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn get_mem_info() -> String {
    let mem_total = mem_info().get("MemTotal").copied().unwrap_or(0) as f64;
    let mem_available = mem_info().get("MemAvailable").copied().unwrap_or(0) as f64;
    let mem_used = mem_total - mem_available;

    let percentage = if mem_total > 0.0 {
        (mem_used / mem_total) * 100.0
    } else {
        0.0
    };

    format!(
        "{} / {} ({})",
        utils::format_bytes(mem_used as u64),
        utils::format_bytes(mem_total as u64),
        theme::colorize(&format!("{:.0}%", percentage)),
    )
}

pub fn get_mem_usage() -> String {
    let mem_total = mem_info().get("MemTotal").copied().unwrap_or(0) as f64;
    let mem_available = mem_info().get("MemAvailable").copied().unwrap_or(0) as f64;

    let percentage = if mem_total > 0.0 {
        ((mem_total - mem_available) / mem_total) * 100.0
    } else {
        0.0
    };

    let bar_len = 20;
    let filled = (percentage / 100.0 * bar_len as f64).round() as usize;
    let empty = bar_len - filled;
    
    format!(
        "[{}{}]",
        theme::colorize(&"#".repeat(filled)),
        ".".repeat(empty)
    )
}
