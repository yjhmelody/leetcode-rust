#![allow(dead_code)]

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = nums.len() as i32 - k + 1;
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;
    let mut pivot;
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
    let pivot_val = nums[left as usize];
    let mut i = left as usize;
    let mut j = right as usize;
    while i < j {
        while i < j && nums[j] >= pivot_val {
            j -= 1;
        }
        nums[i] = nums[j];

        while i < j && nums[i] <= pivot_val {
            i += 1;
        }
        nums[j] = nums[i];
    }

    nums[i] = pivot_val;
    i as i32
}

fn partition2(nums: &mut Vec<i32>, left: i32, right: i32) -> i32 {
    let left = left as usize;
    let right = right as usize;
    let pivot_val = nums[left];
    nums.swap(left as usize, right as usize);
    let mut store_index = left;
    for i in left..right {
        if nums[i] < pivot_val {
            nums.swap(store_index, i);
            store_index += 1;
        }
    }

    nums.swap(right, store_index);
    store_index as i32
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

        let nums = vec![3, 2, 1, 5, 6, 4];
        let val = find_kth_largest(nums, 2);
        assert_eq!(val, 5);

        let nums = vec![-1, -1];
        let val = find_kth_largest(nums, 2);
        assert_eq!(val, -1);
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 1, 4, 5, 6];
        let len = nums.len() as i32 - 1;
        partition2(&mut nums, 0, len);
        assert_eq!(nums, vec![2, 1, 3, 4, 5, 6]);
    }
}
