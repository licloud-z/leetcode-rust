impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        let mut num = num;
        let answer = 0;
        if (num / 10 == 0) || (num / 100 == 0 && num % 100 > 25) { return 1; }
        else if num % 100 > 25 || (num % 100) / 10 == 0{
            Solution::translate_num(num / 10)
        }else {
            Solution::translate_num(num / 10) + Solution::translate_num(num / 100)
        }
    }
}