use colored::*;

pub fn colorize(label: &str) -> String {
    label.red().bold().to_string()
}