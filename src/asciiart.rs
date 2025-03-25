use std::fs;

pub fn get_asciiart(os: &str) -> String {
    let os_lower = os.to_lowercase();

    let art_file = if os_lower.contains("debian") {
        "debian.txt"
    } else {
        "rustyfetch.txt"
    };

    let art_path = format!("asciiart/{}", art_file);
    let art_content = match fs::read_to_string(&art_path) {
        Ok(content) => content,
        Err(_) => "Error loading ascii art".to_string()
    };

    art_content.to_string()
}
