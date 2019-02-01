#![allow(dead_code)]

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    if nums1.is_empty() || nums2.is_empty() {
        return vec![];
    }
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res = vec![];
    for num in nums1 {
        match map.get_mut(&num) {
            Some(v) => { *v += 1; }
            None => { map.insert(num, 1); }
        }
    }

    for num in nums2 {
        match map.get_mut(&num) {
            Some(v) => {
                if *v > 0 {
                    *v -= 1;
                    res.push(num);
                }
            }

            None => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersect(nums1, nums2), vec![2, 2]);


        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersect(nums1, nums2), vec![9, 4]);
    }
}