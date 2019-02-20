#![allow(dead_code)]

// Permutations: count = n * (n-1)
pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let mut count: i32 = 0;
    for i in 0..points.len() {
        for j in 0..points.len() {
            let x_diff = points[i][0] - points[j][0];
            let y_diff = points[i][1] - points[j][1];
            let dis = x_diff * x_diff + y_diff * y_diff;
            match map.get_mut(&dis) {
                Some(v) => *v += 1,
                None => {
                    map.insert(dis, 1);
                }
            }
        }
        let sum: i32 = map.iter().map(|(_, &x)| x * (x - 1)).sum();
        count += sum;
        map.clear();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
        assert_eq!(number_of_boomerangs(points), 2);

        let points = vec![vec![0, 0], vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
        assert_eq!(number_of_boomerangs(points), 20);

        let points = vec![vec![0, 0], vec![2, 1], vec![2, 0], vec![0, 1]];
        assert_eq!(number_of_boomerangs(points), 0);
    }
}
