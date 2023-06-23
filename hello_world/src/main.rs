fn greet() -> String {
    String::from("Hello, world!")
}

fn get_local_disks() -> [&'static str; 3] {
    return ["C", "F", "D"];
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::get_local_disks;
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }

    #[test]
    fn test_get_local_disks() {
        let disks = get_local_disks();
        assert!(disks.len() > 0);
    }
}
