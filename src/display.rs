pub fn format_output(logo: String, infos: Vec<String>) -> String {
    let logo_lines: Vec<&str> = logo.lines().collect();
    let logo_width = logo_lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_lines = logo_lines.len().max(infos.len());
    let diff = max_lines - logo_lines.len().min(infos.len());
    let top = (diff - (diff % 2)) / 2;

    let mut bloc_to_ajust = vec![""; max_lines];
    for (i, info) in infos.iter().enumerate() {
        bloc_to_ajust[top + i] = info;
    }

    let mut output = String::new();
    for i in 0..max_lines {
        let logo_line = logo_lines.get(i).unwrap_or(&"");
        let info_line = bloc_to_ajust.get(i).unwrap_or(&"");
        output.push_str(&format!("{:<width$} {}\n", logo_line, info_line, width = logo_width + 3));
    }
    
    output
}
