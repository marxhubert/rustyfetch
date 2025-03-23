use std::fs;

pub fn get_os() -> String {
    "Linux".to_string()
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
