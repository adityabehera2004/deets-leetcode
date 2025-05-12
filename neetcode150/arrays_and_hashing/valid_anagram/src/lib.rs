pub fn valid_anagram(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    
    s_chars.sort();
    t_chars.sort();
    
    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert_eq!(valid_anagram("racecar".to_string(), "carrace".to_string()), true, "Example 1 failed");
        assert_eq!(valid_anagram("jar".to_string(), "jam".to_string()), false, "Example 2 failed");
    }
}
