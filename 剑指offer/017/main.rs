impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        let mut answer = Vec::new();
        let max = 10_i32.pow(n as u32);
        for i in 1..max{
            answer.push(i);
        }
        answer
        //(1..10_i32.pow(n as u32)).collect()
    }
}