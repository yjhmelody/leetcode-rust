#![allow(dead_code)]

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    use std::collections::HashSet;
    use std::collections::LinkedList;

    let mut visited = HashSet::new();
    let mut dead_ends = HashSet::new();
    let target = target.into_bytes();
    for s in deadends {
        dead_ends.insert(s.into_bytes());
    }
    let mut q = LinkedList::new();
    q.push_back((String::from("0000").into_bytes(), 0));

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        match dead_ends.get(&cur.0) {
            Some(_) => {
                continue;
            }
            None => {}
        }

        match visited.get(&cur.0) {
            Some(_) => {
                continue;
            }
            None => {
                visited.insert(cur.0.clone());
            }
        }
        if target == cur.0 {
            return cur.1;
        }
        let neighbors = get_neighbors(cur.0);

        for n in neighbors {
            q.push_back((n, cur.1 + 1));
        }
    }

    -1
}

fn get_neighbors(s: Vec<u8>) -> Vec<Vec<u8>> {
    let mut neighbors = Vec::with_capacity(8);
    let origin = s.clone();
    for (i, &c) in s.iter().enumerate() {
        let n;
        if c == b'9' {
            n = (b'8', b'0');
        } else if c == b'0' {
            n = (b'9', b'1');
        } else {
            n = (c + 1, c - 1);
        }

        let mut new = origin.clone();
        new[i] = n.0;
        neighbors.push(new);

        let mut new = origin.clone();
        new[i] = n.1;
        neighbors.push(new);
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let dead_ends = vec![
            String::from("0201"),
            String::from("0101"),
            String::from("0102"),
            String::from("1212"),
            String::from("2002"),
        ];

        let res = open_lock(dead_ends, String::from("0202"));
        assert_eq!(res, 6);

        let dead_ends = vec![String::from("8888")];

        let res = open_lock(dead_ends, String::from("0009"));
        assert_eq!(res, 1);
    }
}
