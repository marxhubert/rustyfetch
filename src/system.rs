use std::fs;

pub fn get_os() -> String {
    if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn get_hostname() -> String {
    if cfg!(target_os = "linux") {
        match fs::read_to_string("/etc/hostname") {
            Ok(hostname) => hostname.trim().to_string(),
            Err(e) => format!("Unknown (Error: {})", e),
        }
    } else {
        "Unkown OS".to_string()
    }
}

pub fn get_cpu() -> String {
    if cfg!(target_os = "linux") {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(content) => {
                content.lines()
                    .find(|line| line.starts_with("model name"))
                    .map(|line| line.split(':').nth(1).unwrap_or("Unknown").trim().to_string())
                    .unwrap_or("Unknown CPU".to_string())
            }
            Err(e) => format!("Unknown (Error: {})", e),
        }
    } else {
        "Unkown".to_string()
    }
}
