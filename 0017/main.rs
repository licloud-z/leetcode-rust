use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut answer:Vec<String> = Vec::new();
        if digits == "".to_string() { return answer;}
        let mut hashmap:HashMap<String,String> = HashMap::new();
        hashmap.insert("2".to_string(),"abc".to_string());
        hashmap.insert("3".to_string(),"def".to_string());
        hashmap.insert("4".to_string(),"ghi".to_string());
        hashmap.insert("5".to_string(),"jkl".to_string());
        hashmap.insert("6".to_string(),"mno".to_string());
        hashmap.insert("7".to_string(),"pqrs".to_string());
        hashmap.insert("8".to_string(),"tuv".to_string());
        hashmap.insert("9".to_string(),"wxyz".to_string());
        let mut temp_answer = String::new();
        backtrack(&mut answer, &hashmap, &digits, 0, temp_answer);
        answer
    }
}

pub fn backtrack(answers: &mut Vec<String>, hashmap:&HashMap<String,String>, digits:&String, index: usize, mut answer:String){
    if index == digits.len(){
        answers.push(answer);
    }else {
        let current_num = digits.get(index..(index+1)).unwrap().to_string();
        let new_vec = hashmap.get(&current_num).unwrap();
        for i in new_vec.chars() {
            //println!("{}",i);
            let mut new_answer = answer.clone();
            new_answer.push(i);
            backtrack(answers, hashmap, digits, index+1, new_answer);
            //new_answer.pop();
        }
    }
}