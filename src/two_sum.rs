use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut rets = vec![];
    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            rets.push(j);
            rets.push(i as i32);
            break;
        }
        else {
            map.insert(num, i as i32);
        }
    }

    rets
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }
}