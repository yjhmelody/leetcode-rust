#![allow(dead_code)]

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    let add = String::from("+");
    let sub = String::from("-");
    let mul = String::from("*");
    let div = String::from("/");

    for token in tokens {
        if token == add ||
            token == sub ||
            token == mul ||
            token == div {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let c;
            if token == add {
                c = a + b;
            } else if token == sub {
                c = a - b;
            } else if token == mul {
                c = a * b;
            } else {
                c = a / b;
            }

            stack.push(c);
        } else {
            stack.push(token.parse().unwrap());
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ss = vec!["4", "13", "5", "/", "+"];
        let mut tokens = vec![];
        for s in ss {
            tokens.push(String::from(s));
        }

        assert_eq!(eval_rpn(tokens), 6);
    }
}