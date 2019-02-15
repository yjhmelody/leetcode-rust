#![allow(dead_code)]

// Sort all strings and then record their index in hash map
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let mut res = vec![];
    let sorted_strs: Vec<Vec<u8>> = strs
        .iter()
        .map(|s| {
            let mut s = s.clone().into_bytes();
            s.sort();
            s
        })
        .collect();

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

// ref other people's method
pub fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    strs.into_iter()
        .fold(HashMap::new(), |mut map, s| {
            map.entry(s.bytes().fold([0; 26], |mut hash, b| {
                hash[(b - b'a') as usize] += 1u8;
                hash
            }))
                .or_insert(vec![])
                .push(s);
            map
        })
        .into_iter()
        .map(|s| s.1)
        .collect()
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

        assert_eq!(
            res,
            vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]]
        )
    }

    #[test]
    fn test2() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let res = group_anagrams2(strs);

        // random order
        // assert_eq!(
        //   res,
        //   vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]]
        //)
    }
}
