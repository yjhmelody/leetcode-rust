#![allow(dead_code)]

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut count = 0;
    let mut twice = false;
    for i in 0..nums.len() - 1 {
        if nums[i] != nums[i + 1] {
            twice = false;
            count += 1;
            nums[count] = nums[i + 1];
            // twice
        } else if nums[i] == nums[i + 1] && !twice {
            twice = true;
            count += 1;
            nums[count] = nums[i + 1];
        }
    }
    println!("{:?}", nums);
    count as i32 + 1
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut i = 1;
    let mut k = i - 1;
    let mut j = i + 1;

    while j < nums.len() {
        if (nums[j] != nums[i]) || (nums[j] == nums[i] && nums[j] != nums[k]) {
            k += 1;
            nums[i + 1] = nums[j];
            i += 1;
        }
        j += 1;
    }

    (i + 1) as i32
}

// use incremental nature
pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if j < 2 || nums[j] > nums[i - 2] {
            nums[i] = nums[j];
            i += 1;
        }
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);

        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }

    #[test]
    fn test2() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates2(&mut nums), 5);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates2(&mut nums), 7);

        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates2(&mut nums), 5);
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates3(&mut nums), 5);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates3(&mut nums), 7);

        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates3(&mut nums), 5);
    }
}
