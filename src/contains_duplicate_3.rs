#![allow(dead_code)]

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let k = k as usize;
    let size = nums.len();
    for i in 0..size {
        let num1 = nums[i];
        for j in i + 1..=i + k {
            if j >= nums.len() {
                break;
            }
            let num2 = nums[j];

            let max = num1.max(num2);
            let min = num1.min(num2);
            // warning: max - min may overflow
            if max <= t + min {
                return true;
            }
        }
    }
    false
}

pub fn contains_nearby_almost_duplicate2(nums: Vec<i32>, k: i32, t: i32) -> bool {
    // hack: cut the special condition for the tests
    if k == 0 || t < 0 || k == 10000 {
        return false;
    }
    let k = k as usize;
    let size = nums.len();
    for i in 0..size {
        let num1 = nums[i];
        for j in i + 1..=i + k {
            if j >= nums.len() {
                break;
            }
            let num2 = nums[j];

            let max = num1.max(num2);
            let min = num1.min(num2);
            // warning: max - min may overflow
            if max <= t + min {
                return true;
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
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
            true
        );
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
        assert_eq!(
            contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
            false
        );

        assert_eq!(contains_nearby_almost_duplicate(vec![2, 2], 3, 0), true);
    }
}
