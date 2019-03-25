#![allow(dead_code)]

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 1 {
        return -1;
    }
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[left as usize] == target {
            return left;
        } else if nums[right as usize] == target {
            return right;
        } else if nums[left as usize] < nums[mid as usize] {
            // sorted left vec
            if nums[left as usize] < target && target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // sorted right vec
            if nums[mid as usize] < target && target < nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 0), 4);
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 3), -1);
        let nums = vec![1];
        assert_eq!(search(nums, 0), -1);
    }
}
