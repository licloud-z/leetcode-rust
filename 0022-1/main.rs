impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer:Vec<String> = Vec::new();
        backtrack(&mut answer, n, n, &mut String::new());
        answer
    }
}

pub fn backtrack(answers: &mut Vec<String>, right:i32, left:i32, answer: &mut String){
    if right == 0 && left == 0 {
        answers.push((*answer).clone());
        (*answer).clear();
        return;
    }else if right == left {
        //answers.push((*answer).clone());
        //*answer = answer.to_owned() + "(";
        backtrack(answers, right, left-1, &mut (answer.to_owned() + "("));
    }else {
        if left > 0 {
            //*answer = answer.to_owned() + "(";
            backtrack(answers, right, left-1, &mut (answer.to_owned() + "("));
        }
        //*answer = answer.to_owned() + ")";
        backtrack(answers, right-1, left, &mut (answer.to_owned() + ")"));
    }
}