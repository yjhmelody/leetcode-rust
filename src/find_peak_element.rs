#![allow(dead_code)]

// binary search
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    if nums.len() < 2 {
        return 0;
    }
    while left < right {
        let mid = (right + left) / 2;
        if mid > 0 && mid + 1 < nums.len() {
            if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
                return mid as i32;
            }
            if nums[mid] < nums[mid - 1] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if mid == 0 && nums[0] > nums[1] {
            return mid as i32;
        } else if mid == nums.len() - 1 && nums[mid - 1] < nums[mid] {
            return mid as i32;
        } else {
            left = mid + 1;
        }
    }

    return left as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_peak_element(vec![2, 1, 2]), 0);
        assert_eq!(find_peak_element(vec![2, 1]), 0);
        assert_eq!(find_peak_element(vec![1, 2]), 1);
        assert_eq!(find_peak_element(vec![1, 2, 3]), 2);
        assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
