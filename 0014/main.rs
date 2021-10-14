impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let len = strs.len();
        if len == 1 { return strs[0].to_string(); }
        let mut min_len = usize::MAX;
        for i in 0..len {
            let len1 = (&strs[i]).len();
            if len1 < min_len { min_len = len1; }
        }
        let mut sign = 0;
        let mut right = 0;
        for i in 0..min_len {
            let element = (&strs[0]).get(i..(i+1)).unwrap();
            for num in 1..len{
                let other_ele = (&strs[num]).get(i..(i+1)).unwrap();
                if other_ele != element {
                //println!("{}",other_ele);
                    sign = 1;
                    break;
                }
            }
            if sign == 1 {
                right = i;
                break;
            }
            right = i + 1;
        }
        strs[0].get(0..(right)).unwrap().to_string()
    }
}