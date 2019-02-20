#![allow(dead_code)]

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for num in nums {
        match map.get_mut(&num) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(num, 1);
            }
        }
    }
    let mut heap = map
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<BinaryHeap<_>>();

    let mut res = vec![];
    for _ in 0..k {
        heap.pop().map(|(_k, v)| res.push(v));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(top_k_frequent(nums, k), vec![1, 2]);
    }
}
