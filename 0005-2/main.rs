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
        //let mut hashmap:HashMap<i32,i32> = HashMap::new();
        let mut answer_vec = Vec::new();
        let mut length:i32 = 0;
        //let mut old_length:i32 = 0;
        let mut center = -1;
        let mut right = -1;
        for i in 0..new_vec.len() as i32 {
            if right > i && (center * 2) > i {
                let start_len1 = right-i;
                let start_len2 = answer_vec[(center * 2 - i) as usize];
                length = if start_len2 > start_len1 {start_len1} else { start_len2 };
            }else {
                length = 0;
            }
            while (i-length)>=0 && (i+length)<new_vec.len() as i32 {
                if new_vec[(i-length) as usize] == new_vec[(i+length) as usize] {
                    length = length + 1;
                }else {
                    break;
                }
            }
            //hashmap.insert(i as i32,length - 1);
            //old_length=length;
            if (i+length) >= right {
                right=i+length;
                center=i;
            }
            answer_vec.push(length - 1);
            
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
        //println!("{:?}",answer_vec);
        unsafe {s.slice_unchecked((answer_i-answe_len as usize) / 2,(answer_i+answe_len as usize) / 2 ).to_string()}
    }
    
    }