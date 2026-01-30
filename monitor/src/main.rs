use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::mpsc;
use log::info;
use sysinfo::{Disks};
use dioxus::prelude::*;
use dioxus_desktop::{WindowBuilder};
use dioxus_desktop::muda::{Menu};
use dioxus_desktop::trayicon::{Icon, TrayIconBuilder};
use tao::dpi::{LogicalPosition, PhysicalSize};

fn main() {
    info!("Disk Space Monitor 0.1");

    // Create channel to keep tray alive
    let (tx, rx) = mpsc::channel::<KeyCode>();

    std::thread::spawn(move || {
        setup_tray_icon();
        // Keep thread alive by waiting on channel
        let _ = rx.recv();
    });

    LaunchBuilder::new()
        .with_cfg(make_config())
        .launch(app);
}

fn setup_tray_icon() {
    let menu = Menu::new();

    // Use relative path from project root
    if let Ok(icon_data) = std::fs::read("assets/disk_space_monitor_icon.ico") {
        match Icon::from_rgba(icon_data, 256, 256) {
            Ok(icon) => {
                match TrayIconBuilder::new()
                    .with_menu(Box::new(menu))
                    .with_tooltip("Disk Space Monitor")
                    .with_icon(icon)
                    .build()
                {
                    Ok(tray) => {
                        info!("Tray icon created successfully");
                        // Keep tray alive by not dropping it
                        std::mem::forget(tray);
                    },
                    Err(e) => eprintln!("Failed to build tray icon: {:?}", e),
                }
            },
            Err(e) => eprintln!("Failed to load icon: {:?}", e),
        }
    } else {
        eprintln!("Failed to read icon file at assets/disk_space_monitor_icon.ico");
    }
}

fn make_config() -> dioxus_desktop::Config {
    dioxus_desktop::Config::default()
        .with_window(make_window())
}

fn make_window() -> WindowBuilder {
    // let icon_data: &[u8] = include_bytes!("../assets/disk_space_monitor_icon.ico");
    // let icon: Icon = Icon::from_rgba(icon_data.to_vec(), 64, 64)
    //     .expect("Failed to load window icon");
    WindowBuilder::new()
        .with_transparent(false)
        .with_decorations(true)
        .with_resizable(false)
        .with_always_on_top(false)
        .with_position(LogicalPosition::new(600, 300))
        .with_title("Disk Space Monitor")
        .with_inner_size(PhysicalSize::new(600, 250))
        //.with_window_icon(Some(icon))
}

fn app() -> Element {
    let disks: Disks = get_local_disks();
    struct DiskInfo {
        available_space_pct: i32,
        available_space_human_bytes: String
    }
    let mut disk_space_map: HashMap<&OsStr, DiskInfo> = HashMap::new();
    for disk in &disks {
        let free_space_pct: i32 = get_free_disk_space_percentage(
            (disk.total_space() - disk.available_space()) as i64,
            disk.total_space() as i64
        );
        let avail_space_pct: i32 = 100 - free_space_pct;
        println!(
            "{} ({}% used)\n",
            format!(
                "[{:?}]",
                disk.name()
            ),
            avail_space_pct
        );
        disk_space_map.insert(disk.name(), DiskInfo {
            available_space_pct: avail_space_pct,
            available_space_human_bytes: format!(
                "{:.2} GB",
                disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0)
            )
        });
    }
    rsx! {
        Stylesheet { href: asset!("/assets/tailwind.css") }
        Stylesheet { href: asset!("/assets/monitor.css") }
        div { class: "p-4 bg-slate-400 h-screen",
            h2 { class: "text-3xl font-bold mb-3", "Disk Space Monitor" }

            for (disk_name, info) in disk_space_map {
                div { class: "mb-4",
                    h4 {
                        class: "text-lg mb-1",
                        "{disk_name.to_string_lossy()} ({info.available_space_human_bytes} available)"
                    }
                    div {
                        class: "w-full bg-gray-200 rounded-full h-2",
                        div {
                            class: "bg-purple-800 h-2 rounded-full",
                            style: "width: {info.available_space_pct}%"
                        }
                    }
                }
            }
        }
    }
}

fn get_local_disks() -> Disks {
    let disks: Disks = Disks::new_with_refreshed_list();
    disks
}

fn get_free_disk_space_percentage(written_bytes: i64, total_bytes: i64) -> i32 {
    if total_bytes == 0 {
        return 0;
    }
    let free_space: i64 = total_bytes - written_bytes;
    (free_space * 100 / total_bytes) as i32
}

fn get_available_disk_space_percentage(written_bytes: i64, total_bytes: i64) -> i32 {
    if total_bytes == 0 {
        return 0;
    }
    let free_space: i64 = total_bytes - written_bytes;
    ((free_space / total_bytes) * 100) as i32
}

fn get_low_disk_space_drives(disks: Disks, low_space_threshold_percentage: i32) -> Vec<String> {
    let mut low_disk_space_drives: Vec<String> = Vec::new();
    for disk in &disks {
        let written_bytes: i64 = (disk.total_space() - disk.available_space()) as i64;
        let free_disk_space_percentage: i32 = get_free_disk_space_percentage(written_bytes, disk.total_space() as i64);
        if free_disk_space_percentage <= low_space_threshold_percentage {
            low_disk_space_drives.push(format!("{:?}", disk.name()));
        }
    }
    low_disk_space_drives
}

#[cfg(test)]
mod tests {
    use sysinfo::Disks;
    use super::{get_free_disk_space_percentage, get_local_disks, get_low_disk_space_drives};

    #[test]
    fn test_get_local_disks() {
        let disks: Disks = get_local_disks();
        assert!(&disks.len() > &0);
    }

    #[test]
    fn test_get_free_disk_space_percentage() {
        let free_space: i32 = get_free_disk_space_percentage(100, 1000);
        assert_eq!(free_space, 90);
    }

    #[test]
    fn test_get_low_disk_space_drives() {
        let low_space_threshold_percentage: i32 = 50;
        let low_space_drives: Vec<String> = get_low_disk_space_drives(
            get_local_disks(),
            low_space_threshold_percentage
        );
        assert!(&low_space_drives.len() > &0);
    }
}
