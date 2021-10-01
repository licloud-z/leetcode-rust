use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let new_num:Vec<i32> = nums;
        let mut hash_map:HashMap<i32,i32> = HashMap::new();
        let mut i:usize = 0;

        while i <  new_num.len() {
            let new_tar1 = target - new_num[i];
            if hash_map.contains_key( &new_tar1 ) {
                let final1 = hash_map.get(&new_tar1).unwrap();
                let final_num = vec![(*final1),i as i32];
                return final_num;
            }else {
                let new_tar2 = new_num[i];
                hash_map.insert(new_tar2,i as i32);
            }
            i=i+1;
        }
        [].to_vec()
    }
}