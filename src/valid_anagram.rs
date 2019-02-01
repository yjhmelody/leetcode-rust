#![allow(dead_code)]

pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::new();
    for c in s.chars() {
        match map.get_mut(&c) {
            Some(v) => *v += 1,

            None => {
                map.insert(c, 1);
            }
        }
    }
    for c in t.chars() {
        match map.get_mut(&c) {
            Some(v) => {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            }

            None => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
        assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
    }
}
