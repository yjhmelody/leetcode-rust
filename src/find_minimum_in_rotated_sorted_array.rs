#![allow(dead_code)]

pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        // right vec is sorted and min value in left
        if nums[mid] < nums[right] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    nums[left]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
        let nums = vec![1, 2];
        assert_eq!(find_min(nums), 1);
    }
}
