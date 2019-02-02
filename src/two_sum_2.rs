#![allow(dead_code)]

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    while i < j {
        let sum = numbers[i] + numbers[j];
        if sum == target {
            return vec![(i + 1) as i32, (j + 1) as i32];
        } else if sum > target {
            j -= 1;
        } else {
            i += 1;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
