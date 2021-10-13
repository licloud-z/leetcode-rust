impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut new_vec = Vec::new();
    
        for i in 0..num_rows {
            new_vec.push(String::new());
        }
        if num_rows == 1 {return s;}
        for i in 0..s.len() {
            if i % (num_rows as usize* 2 - 2) < num_rows as usize {
                new_vec[i % (num_rows as usize * 2 - 2)].push_str(s.get(i..(i+1)).unwrap());
            }else {
                new_vec[(num_rows*2 -2 - (i as i32 % (num_rows * 2 - 2))) as usize].push_str(s.get(i..(i+1)).unwrap());
            }
        }
        let mut answer = String::new();
        for i in 0..num_rows {
            answer = answer + &new_vec[i as usize];
        }
        answer
    }
}