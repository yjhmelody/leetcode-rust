#![allow(dead_code)]

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32 - 1;
    }
    let mut i = 0;
    let sum: i32 = nums.iter().sum();

    let mut half = 0;
    while i < nums.len() {
        if half * 2 == sum - nums[i] {
            return i as i32;
        }
        half += nums[i];
        i += 1;
    }
    -1
}

pub fn pivot_index2(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32 - 1;
    }
    let mut i = 0;
    let mut right: i32 = nums.iter().sum();

    let mut left = 0;
    while i < nums.len() {
        right -= nums[i];
        if left == right {
            return i as i32;
        }
        left += nums[i];
        i += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(pivot_index(vec![-1, -1, -1, -1, -1, 0]), 2);
        assert_eq!(pivot_index(vec![-1, -1, -1, 0, 1, 1]), 0);
        assert_eq!(pivot_index(vec![0]), 0);
        assert_eq!(pivot_index(vec![1, 0]), 0);
        assert_eq!(pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(pivot_index2(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(pivot_index2(vec![1, 2, 3]), -1);
        assert_eq!(pivot_index2(vec![-1, -1, -1, -1, -1, 0]), 2);
        assert_eq!(pivot_index2(vec![-1, -1, -1, 0, 1, 1]), 0);
        assert_eq!(pivot_index2(vec![0]), 0);
        assert_eq!(pivot_index2(vec![1, 0]), 0);
        assert_eq!(pivot_index2(vec![-1, -1, 0, 1, 1, 0]), 5);
    }
}
