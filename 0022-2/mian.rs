impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer:Vec<String> = Vec::new();
        backtrack(&mut answer, 0, 0, n, &mut String::new());
        answer
    }
}

pub fn backtrack(answers: &mut Vec<String>, right:i32, left:i32, n:i32, answer: &mut String){
    if (*answer).len() == n as usize * 2 {
        answers.push((*answer).clone());
        (*answer).clear();
        return;
    }
    if  left < n {
        //answers.push((*answer).clone());
        //*answer = answer.to_owned() + "(";
        backtrack(answers, right, left+1, n,&mut (answer.to_owned() + "("));
        //println!("{:?}",answer);
    }
    if left > right {
        //*answer = answer.to_owned() + ")";
        backtrack(answers, right+1, left, n,&mut (answer.to_owned() + ")"));
        //*answer = answer.to_owned() + ")";
    }
}
