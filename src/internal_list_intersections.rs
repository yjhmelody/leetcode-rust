#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

pub fn interval_intersection(a: Vec<Interval>, b: Vec<Interval>) -> Vec<Interval> {
    let mut res = vec![];
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i].start > b[j].end || a[i].end < b[j].start {
                continue;
            }

            let start = a[i].start.max(b[j].start);
            let end = a[i].end.min(b[j].end);

            res.push(Interval::new(start, end));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = vec![
            Interval::new(0, 2),
            Interval::new(5, 10),
            Interval::new(13, 23),
            Interval::new(24, 25),
        ];
        let b = vec![
            Interval::new(1, 5),
            Interval::new(8, 12),
            Interval::new(15, 24),
            Interval::new(25, 26),
        ];
        let c = vec![
            Interval::new(1, 2),
            Interval::new(5, 5),
            Interval::new(8, 10),
            Interval::new(15, 23),
            Interval::new(24, 24),
            Interval::new(25, 25),
        ];
        assert_eq!(interval_intersection(a, b), c);
    }
}
