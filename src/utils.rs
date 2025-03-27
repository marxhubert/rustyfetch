use crate::theme;

pub fn format_bytes(input: u64) -> String {
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

pub fn format_bar(used: u64, total: u64) -> String {
    let percentage = if total > 0 { (used as f64 / total as f64) * 100.0 } else { 0.0 };
    let bar_length = 10;
    let filled = (percentage / 100.0 * bar_length as f64).round() as usize;
    let empty = bar_length - filled;

    format!(
        "[{}{}] {} / {} ({})",
        theme::primary(&"#".repeat(filled)),
        ".".repeat(empty),
        format_bytes(used),
        format_bytes(total),
        theme::primary(&format!("{:.0}%", percentage))
    )
}
