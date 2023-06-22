
fn greet() -> String {
    String::from("Hello, world!")
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}