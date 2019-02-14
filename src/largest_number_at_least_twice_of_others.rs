#![allow(dead_code)]

pub fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    if nums.len() == 1 {
        return 0;
    }
    let largest = nums
        .clone()
        .into_iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    for num in nums {
        if num * 2 > largest.1 && num != largest.1 {
            return -1;
        }
    }
    largest.0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
        assert_eq!(dominant_index(vec![0, 0, 0, 1]), 3);
        assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
