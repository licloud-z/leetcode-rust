impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = Vec::new();
    let mut answer = Vec::new();
    let mut nums = nums;
    nums.sort();
    backtrack(&mut answers, &nums, 0, &mut answer, 0);
    answers
    }
}

pub fn backtrack(answers: &mut Vec<Vec<i32>>, nums: &Vec<i32>, index: i32, answer: &mut Vec<i32>, depth:i32){
    if depth == (*nums).len() as i32 {
        (*answers).push((*answer).clone());
        return;
    }
    for i in 0..(*nums).len() {
        let number = (*nums)[i];
        let mut sign = 0;
        for item in (*answer).iter(){
            if number == (*item){ sign = 1; }
        }
        if sign == 0 {
            (*answer).push(number);
            backtrack(answers, nums, i as i32, answer,depth+1);
            (*answer).pop();
        }
        //backtrack(answers, nums, i as i32, answer,depth);
    }
}