#![allow(dead_code)]

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    if matrix[0].is_empty() {
        return matrix.iter().map(|arr| arr[0]).collect();
    }
    let mut res = vec![];
    let (height, width) = (matrix.len(), matrix[0].len());
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, height, 0, width);
    loop {
        for y in y_min..y_max {
            res.push(matrix[x_min][y])
        }
        x_min += 1;
        if x_min == x_max {
            break;
        }
        for x in x_min..x_max {
            res.push(matrix[x][y_max - 1])
        }
        y_max -= 1;
        if y_min == y_max {
            break;
        }
        for y in (y_min..y_max).rev() {
            res.push(matrix[x_max - 1][y])
        }
        x_max -= 1;
        if x_min == x_max {
            break;
        }
        for x in (x_min..x_max).rev() {
            res.push(matrix[x][y_min])
        }
        y_min += 1;
        if y_min == y_max {
            break;
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
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        assert_eq!(spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);

        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12],
                vec![13, 14, 15],
            ]),
            vec![1, 2, 3, 6, 9, 12, 15, 14, 13, 10, 7, 4, 5, 8, 11]
        );
    }
}
