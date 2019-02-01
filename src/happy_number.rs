#![allow(dead_code)]

pub fn is_happy(n: i32) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut n = n;
    loop {
        let mut sum = 0;
        while n != 0 {
            let temp = n % 10;
            n /= 10;
            sum += temp * temp;
        }
        if sum == 1 {
            return true;
        }
        if !set.insert(sum) {
            return false;
        }
        n = sum;
    }
}

pub fn is_happy2(n: i32) -> bool {
    let mut n = n;
    loop {
        let mut sum = 0;
        while n != 0 {
            let temp = n % 10;
            n /= 10;
            sum += temp * temp;
        }
        if sum == 1 {
            return true;
        }
        if sum == 4 {
            return false;
        }
        n = sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(14), false);
    }

    #[test]
    fn test2() {
        assert_eq!(is_happy2(19), true);
        assert_eq!(is_happy2(14), false);
    }
}
