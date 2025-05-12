use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<_> = nums.iter().collect();
    set.len() != nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 3]), true, "Example 1 failed");
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false, "Example 2 failed");
    }
}