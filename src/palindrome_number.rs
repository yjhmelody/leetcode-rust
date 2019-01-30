#![allow(dead_code)]


pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut x = x;
    let mut reversed_num = 0;
    // when x <= reversed_num, x's length is less than or equal to reversed_num's length
    while x > reversed_num {
        reversed_num *= 10;
        reversed_num += x % 10;
        x /= 10;
    }

    if x == reversed_num || x == reversed_num / 10 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(-121), false);
    }
}
