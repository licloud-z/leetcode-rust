use std::collections::VecDeque;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        let mut vecd = VecDeque::new();
        for item in nums{
            if item % 2 == 0 {
                vecd.push_back(item);
            }else {
                vecd.push_front(item);
            }
        }
        Vec::<i32>::from(vecd)
        /*let mut answer = Vec::new();
        for item in vecd {
            answer.push(item);
        }
        answer*/
    }
}