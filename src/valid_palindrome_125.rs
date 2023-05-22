pub fn is_palindrome(s: String) -> bool {
    let s = s.chars().into_iter()
        .filter_map(|c| c.is_alphanumeric().then(|| c.to_ascii_lowercase()));

    return s.clone().eq(s.rev());
}

#[cfg(test)]
mod tests {
    use crate::valid_palindrome_125::is_palindrome;

    #[test]
    fn test() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert!(!is_palindrome("race a car".to_string()));
        assert!(is_palindrome(" ".to_string()));
        assert!(is_palindrome("a".to_string()));
        assert!(!is_palindrome("0P".to_string()));
    }
}