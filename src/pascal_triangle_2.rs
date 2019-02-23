#![allow(dead_code)]

pub fn get_row(mut row_index: i32) -> Vec<i32> {
    row_index += 1;
    if row_index == 1 {
        return vec![1];
    }
    let mut res = vec![1];
    let row_index = row_index as usize;
    for i in 1..row_index {
        for j in (1..=i).rev() {
            if j == i {
                res.push(1);
            } else {
                let sum = res[j - 1] + res[j];
                res[j] = sum;
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
        assert_eq!(get_row(0), vec![1]);
        assert_eq!(get_row(1), vec![1, 1]);
        assert_eq!(get_row(2), vec![1, 2, 1]);
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }
}
