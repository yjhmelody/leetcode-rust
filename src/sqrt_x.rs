#![allow(dead_code)]

// newton method
pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        x
    } else {
        let x = x as f64;
        let mut res = x;
        while res > x / res {
            res = (res + x / res) / 2.0;
            res = res.floor();
        }
        res as i32
    }
}

// newton method using other way of writing
pub fn my_sqrt2(x: i32) -> i32 {
    let x = x as f64;
    let (mut a1, mut a2) = (1.0f64, 0.0f64);

    while (a1 - a2).abs() >= 0.1 {
        a2 = a1;
        a1 = (a1 + x / a1) / 2.0;
    }
    a1 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(99), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(my_sqrt2(8), 2);
        assert_eq!(my_sqrt2(99), 9);
    }
}
