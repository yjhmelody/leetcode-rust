#![allow(dead_code)]

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }
    let mut res = vec![vec![1]];
    let num_rows = num_rows as usize;
    res.resize(num_rows, vec![]);
    for i in 1..num_rows {
        for j in 0..=i {
            if j == 0 || j == i {
                res[i].push(1);
            } else {
                let sum = res[i - 1][j - 1] + res[i - 1][j];
                res[i].push(sum);
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
        assert_eq!(
            generate(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
    }
}
