#![allow(dead_code)]

pub fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    for nums in queries {
        a[nums[1] as usize] += nums[0];
        res.push(
            a.iter()
                .fold(0, |acc, x| if *x % 2 == 0 { acc + *x } else { acc }),
        );
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
            ),
            vec![8, 6, 2, 4]
        );
    }
}
