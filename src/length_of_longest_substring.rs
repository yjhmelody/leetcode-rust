#![allow(dead_code)]

// slide window from left to right
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let mut longest = 0;
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < s.len() {
        if !set.contains(&s.chars().nth(j).unwrap()) {
            set.insert(s.chars().nth(j).unwrap().clone());
            j += 1;
            longest = usize::max(longest, j - i);
        } else {
            set.remove(&s.chars().nth(i).unwrap());
            i += 1;
        }
    }

    longest as i32
}

// opt: move the window more than one step by look up hash map
pub fn length_of_longest_substring2(s: String) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let mut longest = 0;
    let mut i = 0;
    let mut j = 0;
    while j < s.len() {
        if map.contains_key(&s.chars().nth(j).unwrap()) {
            i = usize::max(map[&s.chars().nth(j).unwrap()], i);
        }
        longest = longest.max(j + 1 - i);
        map.insert(s.chars().nth(j).unwrap(), j + 1);
        j += 1;
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(length_of_longest_substring2(String::from("pwwkew")), 3);
    }
}
