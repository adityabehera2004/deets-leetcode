use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let difference = target - num;
        if seen.contains_key(&difference) {
            return vec![seen[&difference], i as i32];
        }
        seen.insert(num, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![3,4,5,6], 7), vec![0,1], "Example 1 failed");
        assert_eq!(two_sum(vec![4,5,6], 10), vec![0,2], "Example 2 failed");
        assert_eq!(two_sum(vec![5,5], 10), vec![0,1], "Example 3 failed");
    }
}