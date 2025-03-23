mod system;
mod display;
mod logo;

fn main() {
    let os = system::get_os();
    let hostname = system::get_hostname();
    let cpu = system::get_cpu();
    let memory = system::get_memory();
    let uptime = system::get_uptime();
    let kernel = system::get_kernel();
    let packages = system::get_packages();
    let shell = system::get_shell();

    let infos = vec![
        format!("OS: {}", os),
        format!("Host: {}", hostname),
        format!("Kernel: {}", kernel),
        format!("Uptime: {}", uptime),
        format!("Packages: {}", packages),
        format!("Shell: {}", shell),
        format!("CPU: {}", cpu),
        format!("Memory: {}", memory),
    ];
    
    let logo = logo::get_logo();
    let output = display::format_output(logo, infos);

    print!("{}", output);
}
