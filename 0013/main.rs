use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let char_array = vec!['I','V','X','L','C','D','M'];
        let score_array = vec![1,5,10,50,100,500,1000];
        let hashmap:HashMap<_,_> = char_array.iter().zip(score_array.iter()).collect();
    
        let length = s.len();
        if length == 0 {return 0;}
        let mut sum = 0;
        let mut i = length ;
        let mut roma1 = Vec::new();
        for c in s.chars() {
            roma1.push(c);
        }
        let index = roma1.get(i-1).unwrap();
        sum = sum + **hashmap.get(index).unwrap();
        i = i - 1;
        while i > 0 {
            let first_index = roma1.get(i).unwrap();
            let first_add = **hashmap.get(first_index).unwrap();
            let second_index = roma1.get(i-1).unwrap();
            let second_add = **hashmap.get(second_index).unwrap();
            if first_add > second_add {
                sum = sum - second_add;
            }else {
                sum = sum + second_add;
            }
            i = i - 1;
        }
    sum
    }
}