use std::collections::HashSet;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
		let mut hashset = HashSet::new();
        for item in nums.iter() {
            if hashset.contains(item){
                return *item;
            }else {
                hashset.insert(*item);
            }
        }
        0
    }
}