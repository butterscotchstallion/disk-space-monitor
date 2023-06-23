use sysinfo::Disk;
use sysinfo::System;
use sysinfo::SystemExt;

fn main() {
    for disk in get_local_disks() {
        println!("{:?}", disk);
    }
}

fn get_local_disks() -> &'static [Disk] {
    let mut sys = System::new_all();
    sys.refresh_all();
    return sys.disks();
}

#[cfg(test)]
mod tests {
    use super::get_local_disks;

    #[test]
    fn test_get_local_disks() {
        let disks = get_local_disks();
        assert!(disks.len() > 0);
    }
}
