#![allow(dead_code)]

// O(n^2): calculus two array's sum firstly and save to hash map, and then eval two sum
// for other two array. When meet sum1 + sum2 == 0, do counting.
pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut sums = HashMap::new();
    let mut count = 0;
    for &num_a in a.iter() {
        for &num_b in b.iter() {
            let sum = num_a + num_b;
            match sums.get_mut(&sum) {
                Some(x) => {
                    *x += 1;
                }
                None => {
                    sums.insert(sum, 1);
                }
            }
        }
    }

    for &num_c in c.iter() {
        for &num_d in d.iter() {
            let sum = -num_c - num_d;
            match sums.get(&sum) {
                Some(x) => {
                    count += *x;
                }
                None => {}
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]);
        assert_eq!(res, 2);
        let res = four_sum_count(vec![-1, -1], vec![-1, 1], vec![-1, 1], vec![1, -1]);
        assert_eq!(res, 6);
    }
}
