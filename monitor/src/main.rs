use human_bytes::human_bytes;
use log::info;
use sysinfo::Disk;
use sysinfo::System;
use sysinfo::SystemExt;

fn main() {
    info!("Disk space monitor 0.1");
    let sys: &mut System = &mut System::new_all();
    let disks: &[Disk] = get_local_disks(sys);
    for disk in disks {
        println!("{:?}", disk);
    }
}

fn get_local_disks(sys: &mut System) -> &[Disk] {
    sys.refresh_all();
    sys.disks()
}

#[cfg(test)]
mod tests {
    use super::get_local_disks;
    use super::human_bytes::human_bytes;
    use super::Disk;
    use super::System;
    use super::SystemExt;

    #[test]
    fn test_get_local_disks() {
        let sys: &mut System = &mut System::new_all();
        let disks: &[Disk] = get_local_disks(sys);
        assert!(disks.len() > 0);
    }

    #[test]
    fn test_get_human_readable_bytes() {
        assert!(human_bytes(699050250240) == "651GB".to_string());
    }
}
