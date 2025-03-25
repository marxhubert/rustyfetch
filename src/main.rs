mod system;
mod display;
mod theme;
mod asciiart;

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
    let cpu = system::get_cpu();
    let memory = system::get_memory();
    let disk = system::get_disk_info();

    let infos = vec![
        user_at_host,
        separator,
        format!("{}: {}", theme::colorize("OS"), os),
        format!("{}: {}", theme::colorize("Host"), hostname),
        format!("{}: {}", theme::colorize("Kernel"), kernel),
        format!("{}: {}", theme::colorize("Uptime"), uptime),
        format!("{}: {}", theme::colorize("Packages"), packages),
        format!("{}: {}", theme::colorize("Shell"), shell),
        format!("{}: {}", theme::colorize("CPU"), cpu),
        format!("{}: {}", theme::colorize("Memory"), memory),
        format!("{}: {}", theme::colorize("Disk (/)"), disk),
    ];
    
    let asciiart = asciiart::get_asciiart(&os);
    let output = display::format_output(asciiart, infos);

    print!("{}", output);
}
