#![allow(dead_code)]

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut temp = nums[0];
    for num in nums {
        if num == temp {
            count += 1;
        } else if count == 0 {
            temp = num;
            count += 1;
        } else {
            count -= 1;
        }
    }

    temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(majority_element(vec![6, 5, 5]), 5);
    }
}
