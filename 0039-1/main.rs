impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = Vec::new();
        let mut answer = Vec::new();
        backtrack(&mut answers, &candidates, target, 0, &mut answer);
        answers
    }
}

pub fn backtrack(answers: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, target: i32, index: i32, answer: &mut Vec<i32>){
    if index == (*candidates).len() as i32 { return;}
    let mut sum = 0;
    for j in 0..(*answer).len() {
        sum = sum + (*answer)[j];
    }
    if target == sum {
        (*answers).push((*answer).clone());
        return;
    }else if target > sum {
        (*answer).push((*candidates)[index as usize]);
        backtrack(answers, candidates, target, index, answer);
        (*answer).pop();
    }
    backtrack(answers, candidates, target, index + 1, answer);
}