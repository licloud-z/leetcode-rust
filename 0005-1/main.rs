use std::collections::HashMap;

impl Solution {
    
pub fn longest_palindrome(s: String) -> String {
    let origin_vec = s.as_bytes();
    let mut new_vec = Vec::new();
    for item in 0..origin_vec.len() {
        new_vec.push(35 as u8);
        new_vec.push(origin_vec[item]);
    }
    new_vec.push(35 as u8);
    //println!("{:?}",new_vec);

    let mut answer_vec = Vec::new();
    let mut length:i32 = 0;
    for i in 0..new_vec.len() as i32 {
        while (i-length)>=0 && (i+length)<new_vec.len() as i32 {
            if new_vec[(i-length) as usize] == new_vec[(i+length) as usize] {
                length = length + 1;
            }else {
                break;
            }
        }
        answer_vec.push(length - 1);
        length = 0;
    }
    let mut answer_i = 0;
    let mut answe_len = 0;
    for j in 0..answer_vec.len() {
        if answer_vec[j] > answe_len {
            answe_len = answer_vec[j];
            answer_i = j;
        }
    }
    //println!("{},{}",answer_i-answe_len as usize,answer_i+answe_len as usize+1);
    
    unsafe {s.slice_unchecked((answer_i-answe_len as usize) / 2,(answer_i+answe_len as usize) / 2 ).to_string()}
}

}