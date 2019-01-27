#![allow(dead_code)]

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut stack = vec![];

    for num in nums2 {
        while stack.len() != 0 && stack[stack.len() - 1] < num {
            map.insert(stack.pop().unwrap(), num);
        }
        stack.push(num);
    }

    let mut rets = vec![];
    for num in nums1 {
        match map.get(&num) {
            Some(next) => rets.push(*next),
            None => rets.push(-1),
        }
    }

    rets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]), vec![-1, 3, -1]);
        assert_eq!(next_greater_element(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
    }
}