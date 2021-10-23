use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        let mut hashmap:HashMap<i32,bool> = HashMap::new();
        let mut another:HashSet<i32> = HashSet::new();
        let mut king_num = 0;
        let mut max_card = 0;
        let mut min_card = 13;
        for item in nums {
            if item == 0 {
                king_num = king_num + 1;
                continue;
            }
            if another.contains(& item) {
                return false;
            }else {
                another.insert(item);
                max_card = std::cmp::max(max_card, item);
                min_card = std::cmp::min(min_card, item);
            }
            if hashmap.contains_key(& item){
                let real = hashmap.get(& item).unwrap();
                if *real && item != 13{
                    hashmap.insert(item + 1, true);
                }else if *real == false && item != 1 {
                    hashmap.insert(item - 1, false);
                }
                hashmap.remove(& item);
            } else {
                if item == 1 {
                    hashmap.insert(2, true);
                    hashmap.insert(i32::MIN, false);
                }else if item == 13 {
                    hashmap.insert(12, false);
                    hashmap.insert(i32::MAX, true);
                }else {
                    hashmap.insert(item + 1, true);
                    hashmap.insert(item - 1, false);
                }
            }
        }
        //println!("{:?}",hashmap);
        //println!("{} {}",max_card,min_card);
        if (hashmap.len() - king_num <=2) && (max_card-min_card) < 5 {
            return true;
        }else {
            return false;
        }
    }
}