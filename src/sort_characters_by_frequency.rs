#![allow(dead_code)]

// sort the array of frequency
pub fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for c in s.chars() {
        match freq.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                freq.insert(c, 1);
            }
        }
    }

    let mut res = String::new();
    let mut freq = freq.iter().collect::<Vec<_>>();
    freq.sort_by(|a, b| (*b).1.partial_cmp((*a).1).unwrap());

    freq.iter().for_each(|pair| {
        for _ in 0..*pair.1 {
            res.push(*pair.0);
        }
    });
    res
}

// add the n char of occurrences of n times to array[n]
pub fn frequency_sort2(s: String) -> String {
    use std::collections::HashMap;
    use std::iter;

    let mut freq = HashMap::new();
    for c in s.chars() {
        match freq.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                freq.insert(c, 1);
            }
        }
    }

    let mut res = String::new();

    let mut counts: Vec<String> = vec![];
    counts.resize(s.len() + 1, String::new());

    for (c, i) in freq.iter() {
        counts[*i].push(*c);
    }

    for (i, s) in counts.iter().enumerate().rev() {
        for c in s.chars() {
            res.push_str(&iter::repeat(c).take(i).collect::<String>());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = frequency_sort(String::from("tree"));
        assert_eq!(
            res == String::from("eert") || res == String::from("eetr"),
            true
        );
    }

    #[test]
    fn test2() {
        let res = frequency_sort2(String::from("tree"));
        assert_eq!(
            res == String::from("eert") || res == String::from("eetr"),
            true
        );
        assert_eq!(
            frequency_sort2(String::from("eeeee")),
            String::from("eeeee")
        );
    }
}
