#![allow(dead_code)]

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut set1 = HashSet::new();

    for num in nums1 {
        set1.insert(num);
    }

    let mut set2 = HashSet::new();
    for num in nums2 {
        set2.insert(num);
    }

    set1.intersection(&set2).map(|x| *x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection(nums1, nums2), vec![2]);
    }
}
