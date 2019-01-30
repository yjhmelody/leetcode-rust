#![allow(dead_code)]

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while i < nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
        i += 1;
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let len = remove_element(&mut nums, 2) as usize;
        nums.resize(len, 0);
        assert_eq!(nums, vec![0, 1, 3, 0, 4]);
    }
}
