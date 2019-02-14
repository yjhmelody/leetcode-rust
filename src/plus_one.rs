#![allow(dead_code)]

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut i = digits.len() - 1;
    digits[i] += 1;
    let mut carry = true;
    while i > 0 {
        if digits[i] >= 10 {
            digits[i] -= 10;
            digits[i - 1] += 1;
        } else {
            carry = false;
            break;
        }
        i -= 1;
    }
    if carry && digits[0] == 10 {
        digits[0] = 0;
        let mut res = vec![1];
        res.extend(digits);
        res
    } else {
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
