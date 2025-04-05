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

    let user_at_host = format!("{}@{}", theme::primary(&user), theme::primary(&hostname));
    let separator = vec!["="; format!("{}@{}", user, hostname).len()].join("");

    let os = system::get_os();
    let kernel = system::get_kernel();
    let uptime = system::get_uptime();
    let packages = system::get_packages();
    let shell = system::get_shell();
    let cpu_info = cpu::get_cpu_info();
    let cpu_usage = cpu::get_cpu_usage();
    let mem_info = memory::get_mem_info();
    let disk_info = disk::get_disk_info();
    let disk_fs = disk::get_fs();
    let network_info = system::get_network();

    let infos = vec![
        user_at_host,
        separator,
        format!("{}: {}", theme::primary("OS"), os),
        format!("{}: {}", theme::primary("Host"), hostname),
        format!("{}: {}", theme::primary("Kernel"), kernel),
        format!("{}: {}", theme::primary("Uptime"), uptime),
        format!("{}: {}", theme::primary("Packages"), packages),
        format!("{}: {}", theme::primary("Shell"), shell),
        format!("{}: {}", theme::primary("CPU"), cpu_info),
        format!("{}: {}", theme::primary("CPU usage"), cpu_usage),
        format!("{}: {}", theme::primary("Memory"), mem_info),
        format!("{}: {}", theme::primary("Disk /"), format!("({}) {}", disk_fs, disk_info)),
        format!("{}: {}", theme::primary("Network"), network_info),
    ];
    
    let asciiart = asciiart::get_asciiart(&os);
    let output = display::format_output(asciiart, infos);

    print!("{}", output);
}
