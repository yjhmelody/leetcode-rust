#![allow(dead_code)]

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for num in nums {
        if set.insert(num) == false {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}
