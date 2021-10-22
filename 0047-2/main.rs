impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = Vec::new();
        let mut answer = Vec::new();
        let mut extra: Vec<bool> = vec![false;nums.len()];
        let mut nums = nums;
        nums.sort();
        backtrack(&mut answers, &nums, 0, &mut answer, 0, &mut extra);
        answers
    }
}

pub fn backtrack(answers: &mut Vec<Vec<i32>>, nums: &Vec<i32>, index: i32, answer: &mut Vec<i32>, depth:i32, extra: &mut Vec<bool>){
    if depth == (*nums).len() as i32 {
        /*
        let mut sign = 0;
        for item in (*answers).iter(){
            if (*answer) == (*item) { sign = 1; }
        }
        if sign == 0 {
            (*answers).push((*answer).clone());
        }
        */
        (*answers).push((*answer).clone());
        return;
    }
    for i in 0..(*nums).len() {
        //if i > 0 && (*nums)[i] == (*nums)[i-1] {continue;}
        let number = (*nums)[i];
        let mut sign = 0;
        //for item in (*extra).iter(){
        //    if i as i32 == (*item){ sign = 1; }
        //}
        if (*extra)[i] == true {continue;}
        if i > 0 && (*nums)[i] == (*nums)[i-1] && (*extra)[i-1] == false {continue;}
        //for sub in 0..(*extra).len() {
        //    if number == (*answer)[sub]{ sign = 1; }
        //}
        if sign == 0 {
            (*answer).push(number);
            (*extra)[i] = true;
            backtrack(answers, nums, i as i32, answer,depth+1, extra);
            (*answer).pop();
            (*extra)[i] = false;
        }
        //backtrack(answers, nums, i as i32, answer,depth);
    }
}