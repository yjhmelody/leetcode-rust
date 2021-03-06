#![allow(dead_code)]

// use two hash maps to judge isomorphic
pub fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<u8, u8> = HashMap::new();
    let mut map2: HashMap<u8, u8> = HashMap::new();
    let s = s.into_bytes();
    let t = t.into_bytes();
    for i in 0..s.len() {
        let c1 = s[i];
        let c2 = t[i];

        if map.contains_key(&c1) {
            let v = map.get_mut(&c1).unwrap();
            if *v != c2 {
                return false;
            }
        } else if map2.contains_key(&c2) {
            return false;
        } else {
            map.insert(c1, c2);
        }
        map2.insert(c2, c1);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            is_isomorphic(String::from("egg"), String::from("add")),
            true
        );
        assert_eq!(is_isomorphic(String::from("ab"), String::from("aa")), false);
    }
}
