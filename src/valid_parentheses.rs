#![allow(dead_code)]

// without hashmap
pub fn is_valid(s: String) -> bool {
    use std::collections::LinkedList;

    if s.is_empty() {
        return true;
    }

    let mut stack = LinkedList::new();
    for ch in s.chars() {
        match stack.back() {
            None => stack.push_back(ch),
            Some(&top) => {
                if top == '(' && ch == ')' {
                    stack.pop_back();
                } else if top == '[' && ch == ']' {
                    stack.pop_back();
                } else if top == '{' && ch == '}' {
                    stack.pop_back();
                } else {
                    stack.push_back(ch);
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("()[]{}");
        assert_eq!(is_valid(s), true);
        let s = String::from("([)]");
        assert_eq!(is_valid(s), false);
        let s = String::from("{[]}");
        assert_eq!(is_valid(s), true);
    }
}
