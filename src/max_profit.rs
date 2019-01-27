#![allow(dead_code)]

// eval the diff prices and plus all the postive
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        0
    } else {
        let mut profit = 0;
        for i in 0..prices.len() - 1 {
            let diff_price = prices[i + 1] - prices[i];
            if diff_price > 0 {
                profit += diff_price;
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![6, 1, 3, 2, 4, 7]), 7);
    }
}
