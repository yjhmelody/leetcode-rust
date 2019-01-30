#![allow(dead_code)]

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = nums.len() as i32 - k + 1;
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;
    let mut pivot = 0i32;
    while i < j {
        pivot = partition(&mut nums, i, j);
        if pivot > k - 1 {
            j = pivot - 1;
        } else if pivot < k - 1 {
            i = pivot + 1;
        } else {
            break;
        }
    }
    nums[k as usize - 1]
}


fn partition(nums: &mut Vec<i32>, left: i32, right: i32) -> i32 {
    let mut pivot_val = nums[left as usize];
    let mut i = left;
    let mut j = right;
    while i < j {
        while i < j && nums[j as usize] >= pivot_val {
            j -= 1;
        }
        nums[i as usize] = nums[j as usize];

        while i < j && nums[i as usize] <= pivot_val {
            i += 1;
        }
        nums[j as usize] = nums[i as usize];
    }

    nums[i as usize] = pivot_val;
    i
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![3, 2, 1, 5, 6, 4];
        let len = nums.len() as i32 - 1;
        partition(&mut nums, 0, len);
        assert_eq!(nums, vec![1, 2, 3, 5, 6, 4]);

        let mut nums = vec![3, 2, 1, 5, 6, 4];
        let val = find_kth_largest(nums, 2);
        assert_eq!(val, 5);

        let mut nums = vec![-1, -1];
        let val = find_kth_largest(nums, 2);
        assert_eq!(val, -1);
    }
}