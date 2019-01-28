#![allow(dead_code)]


pub fn reverse(x: i32) -> i32 {
    use std::i32;

    let mut x = x;
    let mut ret = 0;
    while x != 0 {
        match i32::checked_mul(ret, 10) {
            Some(val) => { ret = val; }
            None => { return 0; }
        };
        match i32::checked_add(ret, x % 10) {
            Some(val) => { ret = val; }
            None => { return 0; }
        };
        x /= 10;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        use std::i32;
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(i32::max_value()), 0);
        assert_eq!(reverse(i32::min_value()), 0);
    }
}