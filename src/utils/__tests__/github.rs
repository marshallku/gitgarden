#[cfg(test)]
mod tests {
    use crate::utils::github::is_valid_github_username;

    #[test]
    fn test_valid_usernames() {
        assert!(is_valid_github_username("marshallku"));
        assert!(is_valid_github_username("octo-cat"));
        assert!(is_valid_github_username("a"));
        assert!(is_valid_github_username("user123"));
        assert!(is_valid_github_username(&"a".repeat(39)));
    }

    #[test]
    fn test_invalid_usernames() {
        assert!(!is_valid_github_username(""));
        assert!(!is_valid_github_username(&"a".repeat(40)));
        assert!(!is_valid_github_username("-leading"));
        assert!(!is_valid_github_username("trailing-"));
        assert!(!is_valid_github_username("has space"));
        assert!(!is_valid_github_username("path/../traversal"));
        assert!(!is_valid_github_username("query?injection=1"));
        assert!(!is_valid_github_username("한글이름"));
    }
}
