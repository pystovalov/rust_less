pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_palindrom(input: &str) -> bool {
    let cleaded_input: String = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase();
    cleaded_input == reverse(&cleaded_input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!(reverse("wizard"), "draziw");
    }
    #[test]
    fn test_is_palindrom() {
        assert!(is_palindrom("A man, a plan, a canal, Panama"));
        assert!(!is_palindrom("Rustancean"));
    }
}
