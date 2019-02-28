#![allow(dead_code)]

pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let s = s.into_bytes();
    // or use fix-length vec
    let mut map = HashMap::new();

    for ch in s.clone() {
        match map.get_mut(&ch) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(ch, 1);
            }
        }
    }

    let mut count = 0;
    for ch in s {
        match map.get(&ch) {
            Some(&v) => {
                if v == 1 {
                    return count;
                }
            }
            None => unreachable!(),
        }
        count += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "leetcode".to_string();
        assert_eq!(first_uniq_char(s), 0);
        let s = "loveleetcode".to_string();
        assert_eq!(first_uniq_char(s), 2);
    }
}
