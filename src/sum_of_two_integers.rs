#![allow(dead_code)]

// consider i32 as [bool;32]
pub fn get_sum(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_sum(a ^ b, (a & b) << 1)
    }
}

pub fn get_sum2(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a;
        a ^= b;
        b = (temp & b) << 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(-4, 3), -1);
        assert_eq!(get_sum(-2, 3), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(get_sum2(1, 2), 3);
        assert_eq!(get_sum2(-4, 3), -1);
        assert_eq!(get_sum2(-2, 3), 1);
    }
}
