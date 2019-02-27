#![allow(dead_code)]

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();
    for num in nums.into_iter().enumerate() {
        match map.get_mut(&num.1) {
            Some(v) => {
                if num.0 - *v <= k as usize {
                    return true;
                } else {
                    *v = num.0;
                }
            }

            None => {
                map.insert(num.1, num.0);
            }
        }
    }

    false
}

pub fn contains_nearby_duplicate2(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashSet;

    // use slides to record k numbers.
    let mut set = HashSet::new();
    let k = k as usize;
    for (i, num) in nums.clone().into_iter().enumerate() {
        if set.contains(&num) {
            return true;
        } else {
            set.insert(num);
            if set.len() > k {
                let temp = nums[i - k];
                set.remove(&temp);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    }

    #[test]
    fn test2() {
        assert_eq!(contains_nearby_duplicate2(vec![1, 2, 3, 1, 2, 3], 2), false);
        assert_eq!(contains_nearby_duplicate2(vec![1, 0, 1, 1], 1), true);
    }
}
