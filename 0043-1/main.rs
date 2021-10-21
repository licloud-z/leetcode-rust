impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let v1 = num1.as_bytes();
        let mut v1: Vec<_> = v1.iter().map(|x| x - 48).collect();
        v1.reverse();
    //println!("{:?}",v1);
        let v2 = num2.as_bytes();
        let mut v2: Vec<_> = v2.iter().map(|x| x - 48).collect();
        v2.reverse();
        let mut v3:Vec<u8> = vec![0; v1.len() + v2.len()];
        let mut step = 0;
        for item1 in v1.iter(){
            let temp = multi(*item1, & v2);
            add(&mut v3, & temp, step);
            step = step + 1;
        }
    //println!("{:?}",v3.len());
    //if v3.len() == 0 { return "0".to_string(); }
        while v3.len() > 0 && v3[v3.len()-1] == 0 {
            v3.pop();
        }
        if v3.len() == 0 { return "0".to_string(); }
        v3.reverse();
        let mut answer = String::new();
        for term in v3.iter() {
            let c = ((*term) + 48) as char;
            answer.push(c);
        }
    //println!("{:?}",v3);
    //"".to_string()
        answer
    }
}

pub fn multi(num: u8, vec: & Vec<u8>) -> Vec<u8>{
    let mut answer = Vec::new();
    let mut sign = 0;
    for item in vec.iter(){
        let new_num = (*item) * num + sign;
        answer.push(new_num % 10);
        sign = new_num / 10;
    }
    if sign != 0 {
        answer.push(sign);
    }
    answer
}

pub fn add(answer: &mut Vec<u8>, vec: & Vec<u8>, num: usize){
    let mut sign = 0;
    for i in num..(num + vec.len()){
        let temp = (*answer)[i] + vec[i - num] + sign;
        (*answer)[i] = temp % 10;
        sign = temp / 10;
    }
    if sign != 0 {
        (*answer)[num + vec.len()] = sign;
    }
}