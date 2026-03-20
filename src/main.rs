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
use std::{env, io::stdout, process::Command, thread};
use sysinfo::{Disks, System};

// Include auto-generated logo match statement
include!(concat!(env!("OUT_DIR"), "/logos.rs"));

fn get_usage_color(pct: f64) -> Color {
    if pct >= 85.0 {
        return Color::Red;
    } else if pct >= 60.0 {
        return Color::Yellow;
    } else {
        return Color::Green;
    }
}

// Helper function to format uptime beautifully
fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{} mins", minutes)
    }
}

// ==========================================
// LINUX GPU PROBER
// ==========================================
#[cfg(target_os = "linux")]
fn get_gpu_data() -> (String, String, f64) {
    let mut name = String::from("Unknown");
    let mut util_str = String::from("0%");
    let mut util_pct = 0.0;

    if let Ok(output) = Command::new("sh").arg("-c").arg("lspci | grep -E 'VGA|3D' | head -n 1").output() {
        let line = String::from_utf8_lossy(&output.stdout);
        if let Some(start) = line.find(':') {
            let raw_name = line[start + 1..].trim();
            name = raw_name
                .replace("Corporation ", "").replace("Advanced Micro Devices, Inc. ", "")
                .replace("[AMD/ATI] ", "").replace("NVIDIA Corporation ", "").replace("GeForce ", "")
                .split(" (rev").next().unwrap_or(raw_name).to_string();
        }
    }

    if let Ok(output) = Command::new("nvidia-smi").args(["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"]).output() {
        let s = String::from_utf8_lossy(&output.stdout);
        if let Ok(u) = s.trim().parse::<f64>() {
            util_pct = u;
            util_str = format!("{}%", u);
        }
    }

    if name.len() > 14 { name = name.chars().take(14).collect(); }
    (name, util_str, util_pct)
}

// ==========================================
// WINDOWS GPU PROBER (Fixed Null Bytes & Nvidia Priority)
// ==========================================
#[cfg(target_os = "windows")]
fn get_gpu_data() -> (String, String, f64) {
    let mut name = String::from("Unknown");
    let mut util_str = String::from("N/A");
    let mut util_pct = 0.0;

    // 1. Try NVIDIA SMI first (Fastest and most accurate if present)
    if let Ok(output) = Command::new("nvidia-smi").args(["--query-gpu=name,utilization.gpu", "--format=csv,noheader,nounits"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() == 2 {
                name = parts[0].replace("NVIDIA GeForce ", "").replace("NVIDIA ", "");
                if let Ok(u) = parts[1].trim().parse::<f64>() {
                    util_pct = u;
                    util_str = format!("{}%", u);
                }
                if name.len() > 14 { name = name.chars().take(14).collect(); }
                return (name, util_str, util_pct);
            }
        }
    }

    // 2. Fallback to PowerShell (Modern replacement for deprecated WMIC)
    if let Ok(output) = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name",
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                name = trimmed
                    .replace("AMD Radeon ", "")
                    .replace("Intel(R) ", "")
                    .to_string();
                break;
            }
        }
    }

    if name.len() > 14 { name = name.chars().take(14).collect(); }
    (name, util_str, util_pct)
}

// Fallback for macOS / BSD
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn get_gpu_data() -> (String, String, f64) {
    (String::from("Unknown"), String::from("N/A"), 0.0)
}

// ==========================================
// MAIN ENGINE
// ==========================================
fn main() -> std::io::Result<()> {
    let gpu_thread = thread::spawn(|| get_gpu_data());
    let ip_thread = thread::spawn(|| local_ip().map_or("Offline".to_string(), |ip| ip.to_string()));

    let mut sys = System::new_all(); 
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    // Tiny sleep trick required on Windows to give the CPU time to calculate a usage delta
    if cfg!(target_os = "windows") { std::thread::sleep(std::time::Duration::from_millis(15)); sys.refresh_cpu_usage(); }
    
    let disks = Disks::new_with_refreshed_list();
    let mut t_d = 0; let mut a_d = 0;
    for d in &disks { t_d += d.total_space(); a_d += d.available_space(); }

    let (disk_str, disk_p) = if t_d > 0 {
        let u = (t_d - a_d) as f64;
        let p = (u / t_d as f64) * 100.0;
        (format!("{:.1}%", p), p)
    } else { ("N/A".to_string(), 0.0) };

    let ram_p = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
    
    let global_cpu_p = sys.global_cpu_info().cpu_usage() as f64;
    let cpu_name = sys.cpus().first().map_or("Unknown CPU", |c| c.name());
    
    let ram = format!("{:.1}/{:.1} GB", sys.used_memory() as f64 / 1.073e9, sys.total_memory() as f64 / 1.073e9);
    let uptime = format_uptime(System::uptime());
    let os = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let user_host = format!("{} AT {}", whoami::username().to_uppercase(), whoami::fallible::hostname().unwrap_or_default().to_uppercase());

    let kernel = if cfg!(target_os = "windows") {
        format!("Build {}", System::kernel_version().unwrap_or_default())
    } else {
        System::kernel_version().unwrap_or_default()
    };

    let shell = env::var("SHELL").or_else(|_| env::var("COMSPEC")).unwrap_or_default();
    let shell_clean = shell.split(&['/', '\\'][..]).last().unwrap_or("Unknown").replace(".exe", "");
    let term = env::var("TERM").unwrap_or_else(|_| if cfg!(target_os = "windows") { "Win Console".to_string() } else { "Unknown".to_string() });

    #[cfg(target_os = "windows")]
    let (load_title, load_val) = ("󰏗 CPU%", format!("{:.1}%", global_cpu_p));
    
    #[cfg(not(target_os = "windows"))]
    let (load_title, load_val) = ("󰏗 LOAD", format!("{:.2}", System::load_average().one));

    let os_id = os.to_lowercase().replace(" linux", "").replace(" gnu/linux", "").replace(" windows", "");
    let logo = get_logo(&os_id);
    let logo_l = logo.lines().count() as u16;

    let (g_n, g_u, g_p) = gpu_thread.join().unwrap_or((String::from("Unknown"), String::from("0%"), 0.0));
    let ip = ip_thread.join().unwrap_or_else(|_| "Offline".to_string());

    enable_raw_mode()?;
    let mut terminal = ratatui::Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions { viewport: Viewport::Inline(logo_l + 12) }
    )?;

    terminal.draw(|f| {
        let size = f.size();
        let outer = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(4), Constraint::Min(0), Constraint::Length(4)]).split(size);
        let center = outer[1];
        let divider = Line::from("─".repeat((center.width as f32 * 0.7) as usize)).centered().dark_gray();

        let rows = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Length(1), Constraint::Length(logo_l), Constraint::Length(1),
            Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1),
        ]).split(center);

        let t_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Ratio(1, 6); 6]).split(rows[3]);
        let b_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Ratio(1, 6); 6]).split(rows[4]);

        let d_b = |t: &str, c: &str, clr: Color| {
            Paragraph::new(Line::from(c.to_string()).centered()).block(Block::default().title(format!(" {} ", t)).borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(clr)))
        };

        f.render_widget(Paragraph::new(logo).cyan(), rows[1]);
        
        f.render_widget(d_b(" CPU", cpu_name, get_usage_color(global_cpu_p)), t_cols[0]);
        f.render_widget(d_b(load_title, &load_val, Color::Blue), t_cols[1]); 
        f.render_widget(d_b("󰘚 RAM", &ram, get_usage_color(ram_p)), t_cols[2]);
        f.render_widget(d_b("󰢮 GPU", &g_n, Color::Cyan), t_cols[3]);
        f.render_widget(d_b("󰢮 GPU%", &g_u, get_usage_color(g_p)), t_cols[4]);
        f.render_widget(d_b("󰋊 DISK", &disk_str, get_usage_color(disk_p)), t_cols[5]);

        f.render_widget(d_b("󰍹 OS", &os, Color::Blue), b_cols[0]);
        f.render_widget(d_b(" KERNEL", &kernel, Color::Magenta), b_cols[1]); 
        f.render_widget(d_b("󰔟 UPTIME", &uptime, Color::Green), b_cols[2]);
        f.render_widget(d_b(" SHELL", &shell_clean, Color::Green), b_cols[3]);
        f.render_widget(d_b(" TERM", &term, Color::Cyan), b_cols[4]);
        f.render_widget(d_b("󰩟 IP", &ip, Color::Yellow), b_cols[5]);

        f.render_widget(Paragraph::new(divider.clone()), rows[5]);
        f.render_widget(Paragraph::new(Line::from(user_host).centered().bold().cyan()), rows[6]);
        f.render_widget(Paragraph::new(divider), rows[7]);
    })?;

    disable_raw_mode()?;
    println!();
    Ok(())
}