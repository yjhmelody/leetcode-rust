#![allow(dead_code)]

// double pointers
pub fn reverse_vowels(s: String) -> String {
    if s.len() < 1 {
        return s;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut res = s.into_bytes();
    while i < j {
        while i < res.len() && !is_vowel(res[i]) {
            i += 1;
        }
        while j > 0 && !is_vowel(res[j]) {
            j -= 1;
        }
        if i < j {
            res.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}

// other express method for double pointers
pub fn reverse_vowels2(s: String) -> String {
    if s.len() < 1 {
        return s;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut res = s.into_bytes();
    while i < j {
        if !is_vowel(res[i]) {
            i += 1;
            continue;
        }
        if !is_vowel(res[j]) {
            j -= 1;
            continue;
        }
        if i < j {
            res.swap(i, j);
        }
        i += 1;
        j -= 1;
    }
    unsafe { String::from_utf8_unchecked(res) }
}

fn is_vowel(c: u8) -> bool {
    match c {
        b'a' | b'A' | b'e' | b'E' | b'i' | b'I' | b'o' | b'O' | b'u' | b'U' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            reverse_vowels(String::from("leetcode")),
            String::from("leotcede")
        );

        assert_eq!(reverse_vowels(String::from(".")), String::from("."));
    }

    #[test]
    fn test2() {
        assert_eq!(
            reverse_vowels2(String::from("leetcode")),
            String::from("leotcede")
        );

        assert_eq!(reverse_vowels2(String::from(".")), String::from("."));
    }
}
