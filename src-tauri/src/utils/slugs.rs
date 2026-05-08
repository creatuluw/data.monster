pub fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slug() {
        assert_eq!(generate_slug("My Query"), "my-query");
    }

    #[test]
    fn test_special_chars_removed() {
        assert_eq!(generate_slug("Sales & Revenue!"), "sales--revenue");
    }

    #[test]
    fn test_already_slug() {
        assert_eq!(generate_slug("already-a-slug"), "already-a-slug");
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(generate_slug("HELLO WORLD"), "hello-world");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(generate_slug("hello   world"), "hello---world");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(generate_slug(""), "");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(generate_slug("Query 123"), "query-123");
    }
}
