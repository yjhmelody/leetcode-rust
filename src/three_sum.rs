#![allow(dead_code)]

// sort and three pointers
// let target = nums[j]
// look up i, k when nums[i] + nums[j] = target
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let mut res = vec![];

    let mut j = nums.len() - 1;
    while j >= 2 {
        let mut i = 0;
        let mut k = j - 1;
        while i < k {
            let sum = nums[i] + nums[k];
            if sum + nums[j] < 0 {
                i += 1;
            } else if sum + nums[j] > 0 {
                k -= 1;
            } else {
                res.push(vec![nums[i], nums[k], nums[j]]);
                loop {
                    i += 1;
                    if i >= k || nums[i - 1] != nums[i] {
                        break;
                    }
                }
                loop {
                    k -= 1;
                    if i >= k || nums[k + 1] != nums[k] {
                        break;
                    }
                }
            }
        }
        loop {
            j -= 1;
            if j < 2 || nums[j + 1] != nums[j] {
                break;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

        let res = three_sum(vec![0, 1, 2, 3]);
        let right: Vec<Vec<i32>> = vec![];
        assert_eq!(res, right);

        let res = three_sum(vec![0, 0, 0]);
        assert_eq!(res, vec![vec![0, 0, 0]]);

        let res = three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]);
        assert_eq!(
            res,
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-2, -2, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, 0, 2]
            ]
        );
    }
}
