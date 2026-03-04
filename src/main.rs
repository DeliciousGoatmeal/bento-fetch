use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use local_ip_address::local_ip;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    TerminalOptions, Viewport,
};
use std::{env, fs, io::stdout, process::Command, thread};
use sysinfo::{Disks, System};

include!(concat!(env!("OUT_DIR"), "/logos.rs"));

fn get_usage_color(pct: f64) -> Color {
    if pct >= 85.0 { Color::Red } else if pct >= 60.0 { Color::Yellow } else { Color::Green }
}

fn get_gpu_data() -> (String, String, f64) {
    let mut name = String::from("Unknown");
    let mut util_str = String::from("N/A");
    let mut util_pct = 0.0;

    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name,utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() == 2 {
                name = parts[0].replace("NVIDIA GeForce ", "").replace("NVIDIA ", "");
                if let Ok(u) = parts[1].parse::<f64>() {
                    util_pct = u;
                    util_str = format!("{}%", u);
                }
                return (name, util_str, util_pct);
            }
        }
    }

    if let Ok(util_data) = fs::read_to_string("/sys/class/drm/card0/device/gpu_busy_percent") {
        if let Ok(u) = util_data.trim().parse::<f64>() {
            util_pct = u;
            util_str = format!("{}%", u);
        }
        if let Ok(output) = Command::new("lspci").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("VGA") || line.contains("3D") {
                    if let Some(start) = line.rfind('[') {
                        if let Some(end) = line.rfind(']') {
                            name = line[start+1..end].replace("Radeon ", "");
                        }
                    }
                    break;
                }
            }
        }
        return (name, util_str, util_pct);
    }

    if let Ok(output) = Command::new("lspci").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("VGA") || line.contains("3D") {
                if let Some(start) = line.rfind('[') {
                    if let Some(end) = line.rfind(']') {
                        name = line[start+1..end].to_string();
                    }
                }
                break;
            }
        }
    }

    if name.len() > 14 { name = name.chars().take(14).collect(); }
    (name, util_str, util_pct)
}

fn main() -> std::io::Result<()> {
    // THREAD 1: GPU Probe (High Syscall Latency)
    let gpu_thread = thread::spawn(|| get_gpu_data());

    // THREAD 2: Network IP Socket (Medium Syscall Latency)
    let ip_thread = thread::spawn(|| local_ip().map_or("Offline".to_string(), |ip| ip.to_string()));

    // MAIN THREAD: Pure memory reads (Lightning Fast)
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    
    let disks = Disks::new_with_refreshed_list();
    let mut total_disk = 0;
    let mut avail_disk = 0;
    for disk in &disks {
        total_disk += disk.total_space();
        avail_disk += disk.available_space();
    }

    let (disk_str, disk_pct) = if total_disk > 0 {
        let used = (total_disk - avail_disk) as f64;
        let pct = (used / total_disk as f64) * 100.0;
        (format!("{:.1}%", pct), pct)
    } else {
        ("N/A".to_string(), 0.0)
    };

    let ram_pct = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
    let cpu_pct = sys.cpus().first().map_or(0.0, |c| c.cpu_usage()) as f64;
    
    let cpu = sys.cpus().first().map_or("Unknown", |c| c.name()).to_string();
    let ram = format!("{:.1} GB / {:.1} GB", sys.used_memory() as f64 / 1_073_741_824.0, sys.total_memory() as f64 / 1_073_741_824.0);
    let uptime = format!("{} hrs", System::uptime() / 3600);
    let os = System::name().unwrap_or_else(|| "Linux".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    
    let user = whoami::username().to_uppercase();
    let host = whoami::fallible::hostname().unwrap_or_else(|_| "UNKNOWN".to_string()).to_uppercase();
    let user_host = format!("{} AT {}", user, host);
    
    let shell = env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string()).split('/').last().unwrap_or("Unknown").to_string();
    let term = env::var("TERM").unwrap_or_else(|_| "Unknown".to_string());
    let load_avg = format!("{:.2}", System::load_average().one);

    let os_id = os.to_lowercase().replace(" linux", "").replace(" gnu/linux", "");
    let clean_logo = get_logo(&os_id);
    let logo_lines = clean_logo.lines().count() as u16;

    // --- SYNCHRONIZE ---
    // The main thread is done. Now we just wait for the OS to give us the network and GPU data.
    let (gpu_name, gpu_util_str, gpu_pct) = gpu_thread.join().unwrap_or((String::from("Unknown"), String::from("N/A"), 0.0));
    let ip = ip_thread.join().unwrap_or_else(|_| "Offline".to_string());

    // 2. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = ratatui::Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(logo_lines + 12),
        },
    )?;

    // 3. Render the UI
    terminal.draw(|f| {
        let size = f.size();

        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(0),
                Constraint::Length(4),
            ])
            .split(size);

        let center_area = outer_layout[1]; 

        let divider_width = (center_area.width as f32 * 0.70) as usize;
        let divider_str = "─".repeat(divider_width);
        let divider_line = Line::from(divider_str).centered().dark_gray();

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(logo_lines),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(center_area);

        let top_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 6); 6])
            .split(rows[3]);

        let bottom_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 6); 6])
            .split(rows[4]);

        let draw_box = |title: &str, content: &str, color: Color| {
            Paragraph::new(Line::from(content.to_string()).centered())
                .block(
                    Block::default()
                        .title(format!(" {} ", title))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(color)),
                )
                .style(Style::default().fg(Color::Reset))
        };

        f.render_widget(Paragraph::new(clean_logo).cyan(), rows[1]);

        // HARDWARE ROW 
        f.render_widget(draw_box(" CPU", &cpu, get_usage_color(cpu_pct)), top_cols[0]);
        f.render_widget(draw_box("󰘚 RAM", &ram, get_usage_color(ram_pct)), top_cols[1]);
        f.render_widget(draw_box("󰢮 GPU", &gpu_name, Color::Cyan), top_cols[2]);
        f.render_widget(draw_box("󰢮 GPU%", &gpu_util_str, get_usage_color(gpu_pct)), top_cols[3]); 
        f.render_widget(draw_box("󰋊 DISK", &disk_str, get_usage_color(disk_pct)), top_cols[4]); 
        f.render_widget(draw_box("󰏗 LOAD", &load_avg, Color::Blue), top_cols[5]);

        // SOFTWARE ROW
        f.render_widget(draw_box("󰍹 OS", &os, Color::Blue), bottom_cols[0]);
        f.render_widget(draw_box(" KERNEL", &kernel, Color::Magenta), bottom_cols[1]);
        f.render_widget(draw_box("󰔟 UPTIME", &uptime, Color::Green), bottom_cols[2]); 
        f.render_widget(draw_box(" SHELL", &shell, Color::Green), bottom_cols[3]);
        f.render_widget(draw_box(" TERM", &term, Color::Cyan), bottom_cols[4]);
        f.render_widget(draw_box("󰩟 IP", &ip, Color::Yellow), bottom_cols[5]);

        // Dividers & User Line
        f.render_widget(Paragraph::new(divider_line.clone()), rows[5]);
        f.render_widget(Paragraph::new(Line::from(user_host).centered().bold().cyan()), rows[6]);
        f.render_widget(Paragraph::new(divider_line), rows[7]);
    })?;

    disable_raw_mode()?;
    println!(); 
    Ok(())
}
