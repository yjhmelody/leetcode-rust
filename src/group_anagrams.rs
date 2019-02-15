#![allow(dead_code)]

use crate::sort_colors::sort_colors;

// Sort all strings and then record their index in hash map
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let mut res = vec![];
    let sorted_strs = strs.iter().fold(vec![], |mut acc, s| {
        let mut s = s.clone().into_bytes();
        s.sort();
        acc.push(s);
        acc
    });

    for i in 0..strs.len() {
        let temp = sorted_strs[i].clone();
        match map.get(&temp) {
            None => {
                res.push(vec![]);
                map.insert(temp.clone(), res.len() - 1);
            }
            Some(_) => {}
        }
        let index = map.get(&temp).unwrap();
        res[*index].push(strs[i].clone());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let res = group_anagrams(strs);

        // random order
        assert_eq!(
            res,
            vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]]
        )
    }
}
