pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 || nums.len() == 1 {
        return nums.len() as i32;
    }
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] != nums[i + 1] {
            count += 1;
            nums[count] = nums[i + 1];
        }
    }

    count as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }
}
