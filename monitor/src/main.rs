use log::info;
use sysinfo::{Disks};
use dioxus::prelude::*;

fn main() {
    info!("Disk Space Monitor 0.1");
    let disks: Disks = get_local_disks();
    for disk in &disks {
        println!("{} ({}% free)\n", format!(
            "[{:?}]",
            disk.name()),
            get_free_disk_space_percentage((disk.total_space() - disk.available_space()) as i64,
                                           disk.total_space() as i64)
        );
    }
    launch(app);
}

fn app() -> Element {
    rsx! {
        Stylesheet { href: asset!("assets/monitor.css") }
        h1 {
            "Disk Space Monitor"
        }
        div {
            "hello world!"
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
