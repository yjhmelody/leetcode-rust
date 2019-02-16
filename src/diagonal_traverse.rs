#![allow(dead_code)]

/// When meet following conditions:
/// 1. up and j == n-1: i++
/// 2. up and i == 0: j++
/// 3. down and j == m-1: j++
/// 4. down and j == 0: i++
/// and let up = !up
/// otherwise go up and right or go down and left.
pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    let mut i = 0;
    let mut j = 0;
    let mut res = vec![];
    let m = matrix.len();
    let n = matrix[0].len();
    let mut up = true;
    let level = m + n - 2;
    while i + j < level {
        res.push(matrix[i][j]);

        if up {
            if j == n - 1 {
                i += 1;
                up = !up;
            } else if i == 0 {
                j += 1;
                up = !up;
            } else {
                i -= 1;
                j += 1;
            }
        } else {
            if i == m - 1 {
                j += 1;
                up = !up;
            } else if j == 0 {
                i += 1;
                up = !up;
            } else {
                i += 1;
                j -= 1;
            }
        }
    }
    res.push(matrix[i][j]);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );

        assert_eq!(
            find_diagonal_order(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
            vec![1, 2, 3, 5, 4, 6]
        );
    }
}
