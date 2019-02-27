#![allow(dead_code)]

// todo: alter it to KMP
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        0
    } else if needle.len() > haystack.len() {
        -1
    } else {
        let a = haystack.into_bytes();
        let b = needle.into_bytes();
        for i in 0..=(a.len() - b.len()) {
            let mut j = 0;
            while j < b.len() {
                if a[i + j] != b[j] {
                    break;
                }
                j += 1;
            }
            if j == b.len() {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = "hello".to_string();
        let b = "ll".to_string();
        assert_eq!(str_str(a, b), 2);

        let a = "aaaaa".to_string();
        let b = "bba".to_string();
        assert_eq!(str_str(a, b), -1);
    }
}
