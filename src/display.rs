pub fn format_output(logo: String, infos: Vec<String>) -> String {
    let mut logo_lines: Vec<&str> = logo.lines().collect();
    let logo_width = logo_lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_lines = logo_lines.len().max(infos.len());
    let diff = max_lines - logo_lines.len().min(infos.len());
    let top = diff / 2;

    let mut info_bloc: Vec<String> = infos.clone();

    if logo_lines.len() > infos.len() {
        (0..top).for_each(|_| info_bloc.insert(0, String::new()));
    } else {
        (0..top).for_each(|_| logo_lines.insert(0, ""));
    }

    let mut output = String::new();
    for i in 0..max_lines {
        let logo_line = logo_lines.get(i).unwrap_or(&"");
        let info_line = info_bloc.get(i).map(|s| s.as_str()).unwrap_or("");
        output.push_str(&format!(
            "{:<width$} {}\n",
            logo_line,
            info_line,
            width = logo_width + 3
        ));
    }
    
    output
}
