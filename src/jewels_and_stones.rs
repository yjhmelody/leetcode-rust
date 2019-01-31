#![allow(dead_code)]

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let mut count = 0;
    for ch in j.chars() {
        set.insert(ch);
    }
    for ch in s.chars() {
        if set.contains(&ch) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")), 3);
    }
}