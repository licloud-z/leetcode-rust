impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut temp_answer = 0;
        
        if prices.len() == 0 { return 0; }
        let mut min = prices[0];
        for item in prices {
            temp_answer = std::cmp::max(temp_answer, item - min);
            min = std::cmp::min(min, item);
        }
        temp_answer
    }
}