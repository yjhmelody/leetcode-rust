#![allow(dead_code)]

// rotate array in-place by temp array
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 {
        return;
    }
    let k = k as usize % nums.len();
    let mid = nums.len() - k - 1;
    let mut temp = Vec::new();
    for i in mid + 1..nums.len() {
        temp.push(nums[i]);
    }
    for i in (0..=mid).rev() {
        nums[i + k] = nums[i];
    }
    for i in 0..k {
        nums[i] = temp[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![3, 99, -1, -100]);

        let mut nums = vec![1, 2];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![1, 2]);

        let mut nums = vec![1, 2, 3];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![2, 3, 1]);
    }
}
