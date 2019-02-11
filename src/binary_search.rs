#![allow(dead_code)]

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0i32;
    let mut j = nums.len() as i32 - 1;
    let mut mid = (i + j) / 2;
    while i <= j {
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] > target {
            j = mid - 1;
        } else {
            i = mid + 1;
        }
        mid = (i + j) / 2;
    }

    -1
}

pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 && nums[0] != target {
        return -1;
    }

    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut i = nums.len() / 2;

    while l < r {
        if nums[i] > target {
            r = i - 1;
        } else if nums[i] < target {
            l = i + 1;
        } else {
            return i as i32;
        }
        i = (r + l) / 2;
    }

    if nums[i] != target {
        return -1;
    }

    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![2, 5], 0), -1);
        assert_eq!(search(vec![5], 5), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(search2(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search2(vec![2, 5], 0), -1);
        assert_eq!(search2(vec![5], 5), 0);
    }
}
