#![allow(dead_code)]

// todo: use hashmap
pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums.iter().step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 4, 3, 2];
        let res = array_pair_sum(nums);
        assert_eq!(res, 4);
    }
}
