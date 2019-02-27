#![allow(dead_code)]

pub fn add_binary(a: String, b: String) -> String {
    let mut carry = false;
    let a = a.into_bytes();
    let b = b.into_bytes();
    let mut i = a.len() as i32 - 1;
    let mut j = b.len() as i32 - 1;
    let mut sum = Vec::with_capacity(a.len().max(b.len()));
    while i >= 0 && j >= 0 {
        let ch = (a[i as usize], b[j as usize]);
        if carry {
            if ch.0 == b'1' && ch.1 == b'1' {
                sum.push(b'1');
                carry = true;
            } else if ch.0 == b'0' && ch.1 == b'0' {
                sum.push(b'1');
                carry = false;
            } else {
                sum.push(b'0');
                carry = true;
            }
        } else {
            if ch.0 == b'1' && ch.1 == b'1' {
                sum.push(b'0');
                carry = true;
            } else if ch.0 == b'0' && ch.1 == b'0' {
                sum.push(b'0');
                carry = false;
            } else {
                sum.push(b'1');
                carry = false;
            }
        }
        i -= 1;
        j -= 1;
    }
    while i >= 0 {
        let ch = a[i as usize];
        if carry {
            if ch == b'1' {
                sum.push(b'0');
                carry = true;
            } else {
                sum.push(b'1');
                carry = false;
            }
        } else {
            sum.push(ch);
        }
        i -= 1;
    }

    while j >= 0 {
        let ch = b[j as usize];
        if carry {
            if ch == b'1' {
                sum.push(b'0');
                carry = true;
            } else {
                sum.push(b'1');
                carry = false;
            }
        } else {
            sum.push(ch);
        }
        j -= 1;
    }

    if carry {
        sum.push(b'1');
    }
    sum.reverse();
    unsafe { String::from_utf8_unchecked(sum) }
}

pub fn add_binary2(a: String, b: String) -> String {
    let mut buf = Vec::with_capacity(usize::max(a.len(), b.len()) + 1);
    let mut a = a.into_bytes();
    let mut b = b.into_bytes();
    let mut carry = 0;
    while !(a.is_empty() && b.is_empty()) {
        let mut sum = a.pop().map_or(b'0', |ch| ch) + b.pop().map_or(b'0', |ch| ch) - b'0' + carry;
        if sum > b'1' {
            sum -= 2;
            carry = 1;
        } else {
            carry = 0;
        }
        buf.push(sum);
    }
    if carry > 0 {
        buf.push(b'1')
    }
    buf.reverse();
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!(add_binary(a, b), "100".to_string());
    }

    #[test]
    fn test2() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!(add_binary2(a, b), "100".to_string());

        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!(add_binary2(a, b), "10101".to_string());
    }
}
