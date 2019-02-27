//#![allow(dead_code)]
//
//#[derive(Debug, PartialEq, Eq)]
//pub struct Point {
//    pub x: i32,
//    pub y: i32,
//}
//
//impl Point {
//    #[inline]
//    pub fn new(x: i32, y: i32) -> Self {
//        Point { x, y }
//    }
//}
//
//// todo
//pub fn max_points(points: Vec<Point>) -> i32 {
//    use std::collections::HashMap;
//    use std::f64;
//    if points.len() < 3 {
//        return points.len() as i32;
//    }
//    let mut map = HashMap::new();
//    for p in points {
//        map.get_mut(&p)
//    }
//
//    unimplemented!()
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
