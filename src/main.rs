mod system;
mod display;
mod logo;

fn main() {
    let os = system::get_os();
    let hostname = system::get_hostname();
    let cpu = system::get_cpu();
    let memory = system::get_memory();
    let uptime = system::get_uptime();

    let infos = vec![
        format!("OS: {}", os),
        format!("Host: {}", hostname),
        format!("CPU: {}", cpu),
        format!("Memory: {}", memory),
        format!("Uptime: {}", uptime),
    ];
    
    let logo = logo::get_logo();
    let output = display::format_output(logo, infos);

    print!("{}", output);
}
