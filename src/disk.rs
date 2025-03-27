use std::fs;
use std::process::Command;
use std::collections::HashMap;
use crate::utils;

pub fn get_fs() -> String {
    match fs::read_to_string("/proc/mounts") {
        Ok(content) => {
            content.lines()
                .find(|line| line.contains(" / "))
                .and_then(|line| line.split_whitespace().nth(2))
                .unwrap_or("Error")
                .to_string()
        }
        Err(_) => "Error reading '/proc/mounts'".to_string(),
    }
}

fn disk_info() -> Result<HashMap<String, u64>, String> {
    let df_output = Command::new("df")
        .arg("-B1")
        .arg("/")
        .output()
        .map_err(|_| "Error executing df command".to_string())?;

    if !df_output.status.success() {
        return Err("df command failed".to_string());
    }

    let output = String::from_utf8_lossy(&df_output.stdout);
    let mut used_bytes = 0;
    let mut available_bytes = 0;

    if let Some(line) = output.lines().last() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 4 {
            used_bytes = fields[2].parse::<u64>().unwrap_or(0);
            available_bytes = fields[3].parse::<u64>().unwrap_or(0);
        }
    }

    let total_bytes = used_bytes + available_bytes;
    let percentage = if total_bytes > 0 {
        ((used_bytes as f64 / total_bytes as f64) * 100.0).round() as u64
    } else {
        0
    };

    Ok(HashMap::from([
        ("used_bytes".to_string(), used_bytes),
        ("available_bytes".to_string(), available_bytes),
        ("total_bytes".to_string(), total_bytes),
        ("percentage".to_string(), percentage),
    ]))
}

pub fn get_disk_info() -> String {
    match disk_info() {
        Ok(info) => {
            let used = info.get("total_bytes").copied().unwrap_or(0) - info.get("available_bytes").copied().unwrap_or(0);
            let total = info.get("total_bytes").copied().unwrap_or(0);
            utils::format_bar(used, total)
        }
        Err(_) => "Error fetching disk info".to_string(),
    }
}
