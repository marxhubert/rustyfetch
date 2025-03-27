use std::fs;
use std::collections::HashMap;
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
    let info = mem_info();
    let used = info.get("MemTotal").copied().unwrap_or(0) - info.get("MemAvailable").copied().unwrap_or(0);
    let total = info.get("MemTotal").copied().unwrap_or(0);
    utils::format_bar(used, total)
}
