#![allow(dead_code)]

// Problem solving by checksum
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for num in &nums {
        res ^= num;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number(nums), 4);
    }
}
