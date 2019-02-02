#![allow(dead_code)]

// sliding window
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    let mut count = nums.len() + 1;
    loop {
        let old_i = i;
        let old_j = j;
        if j < nums.len() && sum < s {
            sum += nums[j];
            j += 1;
            continue;
        }
        if sum >= s && count > j - i {
            count = j - i;
        }
        if sum >= s {
            sum -= nums[i];
            i += 1;
        }
        // no move
        if old_i == i && old_j == j {
            break;
        }
    }

    if i == 0 {
        0
    } else {
        count as i32
    }
}

// other expression
pub fn min_sub_array_len2(s: i32, nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    let mut count = nums.len() + 1;
    while i < nums.len() {
        if j < nums.len() && sum < s {
            sum += nums[j];
            j += 1;
        } else {
            sum -= nums[i];
            i += 1;
        }

        if sum >= s {
            count = count.min(j - i);
        }
    }

    if count == nums.len() + 1 {
        0
    } else {
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            min_sub_array_len(15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]),
            2
        );
        assert_eq!(min_sub_array_len(7, vec![3, 2, 1]), 0);
        assert_eq!(min_sub_array_len(7, vec![]), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(min_sub_array_len2(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            min_sub_array_len2(15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]),
            2
        );
        assert_eq!(min_sub_array_len2(7, vec![3, 2, 1]), 0);
        assert_eq!(min_sub_array_len2(7, vec![]), 0);
    }
}
