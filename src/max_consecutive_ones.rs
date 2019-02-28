#![allow(dead_code)]

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut max_count = 0;
    for num in nums {
        if num == 0 {
            max_count = max_count.max(count);
            count = 0;
        } else {
            count += 1;
        }
    }
    max_count.max(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(find_max_consecutive_ones(nums), 3);
    }
}
