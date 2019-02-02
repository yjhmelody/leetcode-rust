#![allow(dead_code)]

// double pointers
pub fn is_palindrome(s: String) -> bool {
    if s.len() < 1 {
        return true;
    }
    let s = s.into_bytes();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if !s[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        if !s[j].is_ascii_alphanumeric() {
            j -= 1;
            continue;
        }
        let c1 = s[i];
        let c2 = s[j];
        if c1.to_ascii_lowercase() != c2.to_ascii_lowercase() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

// reverse string
pub fn is_palindrome2(s: String) -> bool {
    let mut s1 = vec![];
    for c in s.into_bytes() {
        if c.is_ascii_alphanumeric() {
            s1.push(c.to_ascii_lowercase());
        }
    }
    let mut s2 = s1.clone();
    s2.reverse();
    s1 == s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_palindrome(String::from(".,")), true);

        assert_eq!(is_palindrome(String::from("A man, a plan, a canal: Panama")), true);
        assert_eq!(is_palindrome(String::from(".a")), true);
        assert_eq!(is_palindrome(String::from("!!!")), true);
        assert_eq!(is_palindrome(String::from("a.")), true);
    }

    #[test]
    fn test2() {
        assert_eq!(is_palindrome2(String::from("A man, a plan, a canal: Panama")), true);
        assert_eq!(is_palindrome2(String::from(".a")), true);
        assert_eq!(is_palindrome2(String::from("a.")), true);
        assert_eq!(is_palindrome2(String::from(".,")), true);
    }
}