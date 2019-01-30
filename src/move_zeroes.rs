#![allow(dead_code)]

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = 0;
    while i < nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);

        let mut nums = vec![2, 1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }
}
