mod system;
mod display;
mod logo;

fn main() {
    let os = system::get_os();
    let hostname = system::get_hostname();
    let cpu = system::get_cpu();

    let infos = vec![
        format!("OS: {}", os),
        format!("Host: {}", hostname),
        format!("CPU: {}", cpu),
    ];
    
    let logo = logo::get_logo();

    let output = display::format_output(logo, infos);

    print!("{}", output);
}
