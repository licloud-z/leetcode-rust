use::std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hashmap:HashMap<i32, i32> = HashMap::new();
        for item in nums{
            let p = hashmap.entry(item).or_insert(0);
            *p += 1;
        }
        for (&key, &val) in hashmap.iter(){
            if val == 1 {
                return key;
            }
            //println!("{}-{}",key,val);
        }
        0
    }
}