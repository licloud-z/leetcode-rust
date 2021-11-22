use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut hashmap = HashMap::new();
        let s = s.as_bytes();
        for item in s.iter() {
            let p = hashmap.entry(*item).or_insert(0);
            *p += 1;
        }
        for item in s.iter(){
            let key = *hashmap.get(item).unwrap();
            if  key == 1{
                return (*item) as char;
            }
            //println!("{}-{}",key,val);
        }
        ' '
    }
}