impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut new_vec = Vec::new();
    let mut char_vec = Vec::new();
    for i in s.chars() {
        char_vec.push(i);
    }
    for i in 0..num_rows {
        new_vec.push(String::new());
    }
    //println!("{:?}",char_vec);
    if num_rows == 1 {return s;}
    for i in 0..s.len() {
        if i % (num_rows as usize* 2 - 2) < num_rows as usize {
            new_vec[i % (num_rows as usize * 2 - 2)].push(char_vec[i]);
        }else {
            //new_vec[(num_rows*2 -2 - (i as i32 % (num_rows * 2 - 2))) as usize].push(*s.get(i).unwrap());
            new_vec[(num_rows*2 -2 - (i as i32 % (num_rows * 2 - 2))) as usize].push(char_vec[i]);
        }
    }
    //println!("{:?}",new_vec);
    let mut answer = String::new();
    for i in 0..num_rows {
        answer = answer + &new_vec[i as usize];
    }
    answer
    }
}