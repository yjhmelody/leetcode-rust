#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// todo
//pub fn max_points(points: Vec<Point>) -> i32 {
//    use std::collections::HashMap;
//    use std::f64;
//
//    let mut map = HashMap::new();
//    let mut count = 0;
//    for i in 0..points.len() {
//        for j in 0..points.len() {
//            let x_diff = points[i].x - points[j].x;
//            let y_diff = points[i].y - points[j].y;
//            if y_diff == 0 {
//                let max = f64::MAX;
//                match map.get_mut(&max) {
//                    Some(v) => *v += 1,
//                    None => {
//                        map.insert(max, 2);
//                    }
//                }
//                continue;
//            }
//            let prop = x_diff as f64 / y_diff as f64;
//            match map.get_mut(&prop) {
//                Some(v) => *v += 1,
//                None => {
//                    map.insert(prop, 2);
//                }
//            }
//            count = count.max(map.iter().max_by(|(_, &a), (_, &b)| {
//                a.cmp(b)
//            }).unwrap());
//            map.clear();
//        }
//    }
//
//    count
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test1() {
//        let points = vec![
//            Point::new(1, 1),
//            Point::new(3, 2),
//            Point::new(4, 1),
//            Point::new(2, 3),
//            Point::new(1, 4),
//        ];
//        assert_eq!(max_points(points), 4);
//    }
//}
