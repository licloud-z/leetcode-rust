impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = Vec::new();
    let mut answer:Vec<i32> = Vec::new();
    let mut candidates=candidates;
    candidates.sort();
    backtrack(&mut answers, &candidates, target, 0, 0, &mut answer);
    answers
    }
}
pub fn backtrack(answers: &mut Vec<Vec<i32>>, candidates: & Vec<i32>, target: i32, index: i32, top: i32, answer: &mut Vec<i32>){
    let mut num = 0;
    for i in (*answer).iter(){
        num = num + *i;
    }
    if num == target {
        (*answers).push((*answer).clone());
        return;
    }
    if num > target {
        return;
    }
    for i in index..(*candidates).len() as i32 {
        if i>index && (*candidates)[i as usize] == (*candidates)[i as usize-1] {continue;}
        (*answer).push((*candidates)[i as usize]);
        backtrack(answers, candidates, target, i +1, index + 1, answer);
        (*answer).pop();
    }
}