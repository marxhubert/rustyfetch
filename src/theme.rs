use colored::*;

pub fn primary(label: &str) -> String {
    label.red().bold().to_string()
}
