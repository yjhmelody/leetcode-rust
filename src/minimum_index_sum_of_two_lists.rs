#![allow(dead_code)]

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let mut min = 2000;
    let mut res = vec![];
    for (i, x) in list1.into_iter().enumerate() {
        map.insert(x, i);
    }
    for (i, x) in list2.into_iter().enumerate() {
        match map.get_mut(&x) {
            Some(v) => {
                if *v + i < min {
                    min = *v + i;
                    res.clear();
                    res.push(x);
                } else if *v + i == min {
                    res.push(x);
                }
            }
            None => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list1 = vec!["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list1: Vec<String> = list1.into_iter().map(|x| x.to_string()).collect();
        let list2 = vec![
            "Piatti",
            "The Grill at Torrey Pines",
            "Hungry Hunter Steakhouse",
            "Shogun",
        ];
        let list2: Vec<String> = list2.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(find_restaurant(list1, list2), vec!["Shogun"]);

        let list1 = vec!["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list1: Vec<String> = list1.into_iter().map(|x| x.to_string()).collect();
        let list2 = vec!["KFC", "Shogun", "Burger King"];
        let list2: Vec<String> = list2.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(find_restaurant(list1, list2), vec!["Shogun"]);
    }
}
