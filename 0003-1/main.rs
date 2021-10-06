use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut new_vec = Vec::new();
        let mut new_haspmap:HashMap<char,i32> = HashMap::new();
        for char in s.chars() {
            new_vec.push(char);
        }
    //println!("{:?}",new_vec);
        let mut high = 0;
        let mut answer = 0;
        for i in 0..new_vec.len() {
            if high == new_vec.len() {break;}
            let mut new_key = new_vec[high];
            while !new_haspmap.contains_key(&new_key) {
                new_haspmap.insert(new_key,high as i32);
                high = high + 1;
                if high == new_vec.len() {break;}
                new_key = new_vec[high as usize];
            }
            let temp_answer = (high as i32 - i as i32);
            if temp_answer > answer {
                answer = temp_answer;
            }
            new_haspmap.remove(&new_vec[i]);
        }
        answer
    }
}