mod system;
mod display;
mod theme;
mod asciiart;
mod utils;
mod cpu;
mod memory;
mod disk;

fn main() {
    let user = system::get_user();
    let hostname = system::get_hostname();

    let user_at_host = format!("{}@{}", theme::colorize(&user), theme::colorize(&hostname));
    let separator = vec!["="; format!("{}@{}", user, hostname).len()].join("");

    let os = system::get_os();
    let kernel = system::get_kernel();
    let uptime = system::get_uptime();
    let packages = system::get_packages();
    let shell = system::get_shell();
    let cpu_info = cpu::get_cpu_info();
    let cpu_usage = cpu::get_cpu_usage();
    let mem_info = memory::get_mem_info();
    let mem_usage = memory::get_mem_usage();
    let disk_info = disk::get_disk_info();
    let disk_usage = disk::get_disk_usage();

    let infos = vec![
        user_at_host,
        separator,
        format!("{}: {}", theme::colorize("OS"), os),
        format!("{}: {}", theme::colorize("Host"), hostname),
        format!("{}: {}", theme::colorize("Kernel"), kernel),
        format!("{}: {}", theme::colorize("Uptime"), uptime),
        format!("{}: {}", theme::colorize("Packages"), packages),
        format!("{}: {}", theme::colorize("Shell"), shell),
        format!("{}: {}", theme::colorize("CPU"), cpu_info),
        format!("{}: {}", theme::colorize("CPU Usage"), cpu_usage),
        format!("{}: {}", theme::colorize("Memory"), mem_info),
        format!("{}: {}", theme::colorize("Memory Usage"), mem_usage),
        format!("{}: {}", theme::colorize("Disk (/)"), disk_info),
        format!("{}: {}", theme::colorize("Disk Usage"), disk_usage),
    ];
    
    let asciiart = asciiart::get_asciiart(&os);
    let output = display::format_output(asciiart, infos);

    print!("{}", output);
}
