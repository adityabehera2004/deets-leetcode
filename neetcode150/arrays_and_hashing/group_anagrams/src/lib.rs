use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<[i32; 26], Vec<String>> = HashMap::new();
    for s in strs {
        let mut count = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        groups.entry(count).or_insert_with(Vec::new).push(s);
    }
    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn to_comparable(mut lists: Vec<Vec<String>>) -> HashSet<Vec<String>> {
        for group in &mut lists {
            group.sort();
        }
        lists.into_iter().collect()
    }

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            to_comparable(group_anagrams(vec![
                "act".to_string(),
                "pots".to_string(),
                "tops".to_string(),
                "cat".to_string(),
                "stop".to_string(),
                "hat".to_string(),
            ])),
            to_comparable(vec![
                vec!["hat".to_string()],
                vec!["act".to_string(), "cat".to_string()],
                vec!["stop".to_string(), "pots".to_string(), "tops".to_string()],
            ]),
            "Example 1 failed"
        );

        assert_eq!(
            to_comparable(group_anagrams(vec!["x".to_string()])),
            to_comparable(vec![vec!["x".to_string()]]),
            "Example 2 failed"
        );

        assert_eq!(
            to_comparable(group_anagrams(vec!["".to_string()])),
            to_comparable(vec![vec!["".to_string()]]),
            "Example 3 failed"
        );
    }
}
