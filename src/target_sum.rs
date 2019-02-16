#![allow(dead_code)]

// dfs: O(2^n)
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    _find_target_sum_ways(&nums, 0, s, 0)
}

fn _find_target_sum_ways(nums: &Vec<i32>, index: usize, s: i32, sum: i32) -> i32 {
    let mut res = 0;
    if index == nums.len() {
        if sum == s {
            res += 1;
            return res;
        } else {
            return res;
        }
    } else {
        res += _find_target_sum_ways(&nums, index + 1, s, sum + nums[index]);
        res += _find_target_sum_ways(&nums, index + 1, s, sum - nums[index]);
        res
    }
}

// analysis and dp
// sum(P) - sum(N) = target
// sum(P) + sum(N) + sum(P) - sum(N) = target + sum(P) + sum(N)
// 2 * sum(P) = target + sum(nums)
// so, if target + sum(nums) is not even, return 0,
// else get sum(P) ==  (target + sum(nums))/2
pub fn find_target_sum_ways2(nums: Vec<i32>, s: i32) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}
