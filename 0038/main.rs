impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1{
            return "1".to_string();
        }
        let mut answer = "1".to_string();
        for i in 1..n{
            answer = count(answer);
        }
        answer
    }
}

pub fn count(n: String) -> String {
    let mut answer = String::new();
    let n = n.as_bytes();
    let len = n.len();
    if len == 1 {
        answer.push_str(&*("1".to_string()));
        answer.push(n[0] as char);
        return answer;
    }
    let mut last_element = n[0];
    let mut step = 1;

    for i in 1..len {
        let element = n[i];
        
        if element == last_element {
            step = step + 1;
            if i == len - 1 {
                answer.push_str(&*(step.to_string()));
                answer.push(element as char);
                break;
            }
            last_element = element;
        }else {
            answer.push_str(&*(step.to_string()));
            answer.push(last_element as char);
            last_element = element;
            step = 1;
            if i == len - 1{
                answer.push_str(&*("1".to_string()));
                answer.push(element as char);
                break;
            }
        }
    }
    answer
}